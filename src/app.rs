

use std::cell::{Ref, RefCell};
use std::{cell::Cell, io};
use ratatui::{layout::{Constraint, Direction, Layout}, Frame};

use crate::fms::{PathKind, list_files_in_directory};

use crate::header::HeaderWidget;
use crate::body::BodyWidget;
use crate::footer::FooterWidget;

/// アプリケーション状態管理構造体
/// 
/// # フィールド
/// * `dir_path` - 現在のディレクトリパス
/// * `files` - ディレクトリ内のファイル一覧
/// * `focus_file_no` - フォーカスされているファイル番号
/// * `max_file_count_in_page` - 1ページに表示できる最大のファイル数
/// 
pub struct App {
    dir_path: RefCell<String>,
    files: RefCell<Vec<PathKind>>,
    focused_file_index: Cell<usize>,
    max_files_per_page: Cell<usize>,
}

// App構造体の実装
impl App {
    /// 新しいAppインスタンスを返します。
    pub fn new(dir_path: &str) ->io::Result<App> {
        // 初期読み込み
        let files = list_files_in_directory(dir_path)?;
        Ok(App {
            dir_path: RefCell::new(dir_path.to_string()),
            files: RefCell::new(files),
            focused_file_index: Cell::new(0),      
            max_files_per_page: Cell::new(0),      
        })
    }

    /// 現在のディレクトリパスを取得する
    pub fn dir_path(&self) -> String {
        self.dir_path.borrow().to_owned()
    }

    /// ディレクトリパスを設定する
    /// ディレクトリ内のファイル一覧を取得し、フォーカスファイル番号を0に設定します
    pub fn set_dir_path(&self, dir_path: &str) {
        self.dir_path.replace(dir_path.into());
        let files = list_files_in_directory(dir_path).unwrap();
        self.files.replace(files);
        self.focused_file_index.set(0);
    }

    /// ファイル一覧を取得する
    pub fn file_list(&self) -> Ref<Vec<PathKind>> {
        self.files.borrow()
    }

    /// ファイル数を取得する
    pub fn count_file_list(&self) -> usize {
        self.files.borrow().len()
    }

    /// フォーカスされているファイルを取得する
    pub fn focused_file(&self) -> PathKind {
        self.files.borrow()[self.focused_file_index.get()].clone()
    }

    /// フォーカスされているファイル番号を取得する
    pub fn focus_file_no(&self) -> usize {
        self.focused_file_index.get()
    }

    /// フォーカスされているファイル番号を設定する
    pub fn set_focus_file_no(&self, no: usize) {
        self.focused_file_index.set(no);
    }
    /// ファイル一覧のページ内最大表示数を取得する
    pub fn max_files_per_page(&self) -> usize {
        self.max_files_per_page.get()
    }

    /// ファイル一覧のページ内最大表示数を設定する
    pub fn set_max_files_per_page(&self, count: usize) {
        self.max_files_per_page.set(count);
    }

    /// 現在のページNoを取得する
    pub fn current_page_no(&self) -> usize {
        (self.focused_file_index.get() / self.max_files_per_page.get()) + 1
    }

    /// ターミナルに描画する
    pub fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(1)])
            .split(frame.area());


        if let [header, body, footer] = chunks[..] {

            // ボディ描画
            let body_widget = BodyWidget::new(self);
            frame.render_widget(body_widget, body);

            // ヘッダー描画
            // ボディ描画で、最大のファイル表示数を取得して、ヘッダー描画で使用する
            let header_widget = HeaderWidget::new(self);
            frame.render_widget(header_widget, header);

            // フッター描画
            let footer_widget = FooterWidget;
            frame.render_widget(footer_widget, footer);

        }

    }

}
