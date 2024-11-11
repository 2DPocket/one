use std::fs;
use std::io;
use std::path::Path;
use std::io::{Error, ErrorKind};

/// ファイル種別列挙型
#[derive(Debug)]
pub enum PathKind {
    FILE(String),
    DIR(String),
}

/// ディレクトリ内のファイル一覧を取得する
pub fn list_files_in_directory(dir: &str) -> io::Result<Vec<PathKind>> {
    let path = Path::new(dir);
    let mut file_names: Vec<PathKind> = Vec::new();

    if !path.is_dir() {
        return Err(Error::new(ErrorKind::Other, "ディレクトリではない"));
    } else {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name )= entry.file_name().to_str() {
                    file_names.push(PathKind::FILE(file_name.to_owned()));
                } else {
                    println!("{:?}", entry.file_name().to_str());
                    return Err(Error::new(ErrorKind::Other, "ファイ名変換でエラー"));
                }
            } else if path.is_dir() {
                if let Some(file_name )= entry.file_name().to_str() {
                    file_names.push(PathKind::DIR(file_name.to_owned()));
                } else {
                    println!("{:?}", entry.file_name().to_str());
                    return Err(Error::new(ErrorKind::Other, "ファイ名変換でエラー"));
                }
            }  
        }
    }

    Ok(file_names)
}