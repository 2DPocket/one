

use std::io;

use crate::fms::{PathKind, list_files_in_directory};

/// アプリケーション状態管理構造体
pub struct App {
    dir_path: String,
    file_names: Vec<PathKind>,
}

impl App {
    pub fn new(dir_path: &str) ->io::Result<App> {
        // 初期読み込み
        let file_names = list_files_in_directory(dir_path)?;
        let current_dir_path = std::env::current_dir()?;

        Ok(App {
            dir_path: current_dir_path.to_str().unwrap().to_string(),
            file_names: file_names,
        })
    }

    pub fn path(&self) -> &String {
        &self.dir_path
    }

    pub fn file_names(&self) -> &Vec<PathKind> {
        &self.file_names
    }

}