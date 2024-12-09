use std::fs;
use std::fs::Metadata;
use std::io;
use std::path::Path;
use std::io::{Error, ErrorKind};

use chrono::DateTime;
use chrono::Local;
use num_format::Locale;
use num_format::ToFormattedString;
use unicode_width::UnicodeWidthStr;

/// ファイル情報構造体
/// 
/// # フィールド
/// * `file_no` - ファイル番号
/// * `file_name` - ファイル名
/// * `file_size` - ファイルサイズ
/// * `last_modified` - 更新日時
/// 
/// # 例
/// 
#[derive(Clone, Debug)]
pub struct FileInformation {
    pub file_no: u16,
    pub file_name: String,
    pub file_size: u64,
    pub last_modified: Option<DateTime<Local>>,
}

/// ファイル構造体のメソッド実装
impl FileInformation {
    
    /// ファイル名をフォーマットする
    /// ファイル名が指定した桁数を超える場合は切り詰める
    /// ファイル名が指定した桁数に満たない場合はスペースを追加する
    /// ファイル名が指定した桁数の場合はそのまま返す
    /// 
    /// # 引数
    ///
    /// * `max_width` - フォーマと後の最大桁数
    ///
    /// # 例
    ///  
    /// # 戻り値
    /// `String` - フォーマットされた名前
    pub fn format_file_name(&self, max_width: usize) -> String {
        let name_width = UnicodeWidthStr::width(self.file_name.as_str());
        
        if name_width <= max_width {
            // 文字列が短い場合はスペースを追加
            let padding = max_width - name_width;
            format!("{}{}", self.file_name, " ".repeat(padding))
        } else {
            // 文字列が長すぎる場合は切り詰める
            let mut truncated_name = String::new();
            let mut current_width = 0;
            for c in self.file_name.chars() {
                let char_width = UnicodeWidthStr::width(c.to_string().as_str());
                if current_width + char_width > max_width - 2 {
                    break;
                }
                truncated_name.push(c);
                current_width += char_width;
            }
            truncated_name.push('…'); // 省略記号を追加
            let truncated_name_width = UnicodeWidthStr::width(truncated_name.as_str());

            if truncated_name_width < max_width {
                // 省略記号を追加しても幅が足りない場合はスペースを追加
                let padding = max_width - truncated_name_width;
                format!("{}{}", truncated_name, " ".repeat(padding))
            } else {
                truncated_name
            }
        }
    }

    /// ファイルサイズをフォーマットする
    /// 1024バイト以上の場合はKB、MB、GBに変換する
    /// 三桁カンマ区切りでフォーマットする
    /// # 引数
    ///
    /// # 例
    ///  
    /// # 戻り値
    /// `String` - フォーマットされたサイズ
    pub fn format_file_size(&self) -> String {
        let size = self.file_size as f64;
        const GIGABYTE: f64 = 1024.0 * 1024.0 * 1024.0;
        const MEGABYTE: f64 = 1024.0 * 1024.0;
        const KILOBYTE: f64 = 1024.0;

        let formatted_size = if size >= GIGABYTE {
            format!("{:.1}G", (size / GIGABYTE))
        } else if size >= MEGABYTE {
            format!("{:.1}M", (size / MEGABYTE))
        } else if size >= KILOBYTE {
            format!("{:.1}K", (size / KILOBYTE))
        } else {
            format!("{} ", (size as u64).to_formatted_string(&Locale::en))
        };
  
        // 小数点以下が0の場合は小数点以下を削除
        // それ以外は小数点以下を残す
        if let Some((int_part, dec_part)) = formatted_size.split_once('.') {
            let num = int_part.parse::<u64>().unwrap();
            format!("{:>5}.{}", num.to_formatted_string(&Locale::en), dec_part)
        } else {
            format!("{:>8}", formatted_size)
        }
    }

    /// 更新日時をフォーマットする
    /// YYYY/MM/DD HH:MM 形式でフォーマットする 
    /// # 引数
    ///
    /// # 例
    ///  
    /// # 戻り値
    /// `String` - フォーマットされた更新日時
    pub fn formatted_modified(&self) -> String {
        if let Some(modified) = self.last_modified {
            modified.format("%Y/%m/%d %H:%M").to_string()
        } else {
            " ".repeat(16)
        }
    }
}

/// ファイル種別列挙型
/// 
/// # バリアント
/// * `FILE` - ファイル
/// * `DIR` - ディレクトリ
/// 
/// # 例
/// 
#[derive(Clone, Debug)]
pub enum PathKind {
    FILE(FileInformation),   // ファイル
    DIR(FileInformation),    // ディレクトリ  
}

/// ディレクトリ内のファイル一覧を取得します。
/// ファイル名の昇順でソートしますが、ディレクトリが先に来るようにします。
/// 
/// # 引数
/// * `dir` - ディレクトリのパス
/// 
/// # 戻り値
/// `Vec<PathKind>` - ファイル一覧

pub fn list_files_in_directory(dir: &str) -> io::Result<Vec<PathKind>> {
    let directory_path = Path::new(dir);
    let mut file_list: Vec<PathKind> = Vec::new();

    if !directory_path.is_dir() {
        return Err(Error::new(ErrorKind::Other, "指定されたパスはディレクトリではありません"));
    } else {
        // ディレクトリの場合は ..（親ディレクトリ） を追加
        let mut file_no = 0;
        file_list.push(PathKind::DIR(FileInformation {file_no, file_name: "..".to_string(), file_size: 0, last_modified: None}));
        for entry in fs::read_dir(directory_path)? {
            file_no += 1;
            let entry = entry?;
            let path = entry.path();
            let metadata  = entry.metadata()?;
            if let Some(file_name )= entry.file_name().to_str() {
                add_file_info(&mut file_list, file_no, file_name, &metadata, path.is_dir())?;
            } else {
                println!("{:?}", entry.file_name().to_str());
                return Err(Error::new(ErrorKind::Other, "ファイル名の変換に失敗しました"));
            }
        }
    }

    // 名前順で並び替え
    // 但しディレクトリが先に来るようにする
    file_list.sort_unstable_by (|a, b| {
        match (a, b) {
            (PathKind::DIR(_), PathKind::FILE(_)) => std::cmp::Ordering::Less,
            (PathKind::FILE(_), PathKind::DIR(_)) => std::cmp::Ordering::Greater,
            (PathKind::DIR(a), PathKind::DIR(b)) => a.file_name.cmp(&b.file_name),
            (PathKind::FILE(a), PathKind::FILE(b)) => a.file_name.cmp(&b.file_name),
        }
    });

    Ok(file_list)
}

/// ファイル情報を追加する関数
/// 
/// # 引数
/// * `file_list` - ファイル情報を格納するベクター
/// * `file_no` - ファイル番号
/// * `file_name` - ファイル名
/// * `metadata` - ファイルのメタデータ
/// * `is_dir` - ディレクトリかどうかを示すブール値
/// 
/// # 戻り値
/// `io::Result<()>` - 成功した場合は空のタプルを返す
fn add_file_info(file_list: &mut Vec<PathKind>, file_no: u16, file_name: &str, metadata: &Metadata, is_dir: bool) ->io::Result<()>{
    let file_info = FileInformation {
        file_no,
        file_name: file_name.to_string(),
        file_size: metadata.len(),
        last_modified: Some(metadata.modified()?.into()),
    };

    if is_dir {
        file_list.push(PathKind::DIR(file_info));
    } else {
        file_list.push(PathKind::FILE(file_info));
    }

    Ok(())
}