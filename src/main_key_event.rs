
use std::{io, path::Path};

use crossterm::event::KeyEvent;
use ratatui::crossterm::event::KeyCode;

use crate::{app::App, fms::PathKind};

pub enum AppEvent {
    QUIT,
    NONE,
    PROCESSED,

}

pub fn main_key_event(app: &App, key_event: &KeyEvent) -> Result<AppEvent, io::Error> {
    match key_event.code {
        KeyCode::Char('q') => {
            Ok(AppEvent::QUIT)
        }
        KeyCode::Down => {
            let current_focus_no = app.focus_file_no();
            if current_focus_no < app.file_list().len() - 1 {
                app.set_focus_file_no(current_focus_no + 1);
            }
            Ok(AppEvent::PROCESSED)
        }
        KeyCode::Up => {
            let current_focus_no = app.focus_file_no();
            if current_focus_no > 0 {
                app.set_focus_file_no(current_focus_no - 1);
            }
            Ok(AppEvent::PROCESSED)
        }
        KeyCode::Right => {
            // 1画面2分割されてるので、左側にフォーカスがある場合は、行数を足して、右横に移動する
            // 右横に移動した場合は、先頭にフォーカスを当てる
            let current_focus_no = app.focus_file_no();
            let max_file_count_in_page = app.max_files_per_page();
            let max_row_count = max_file_count_in_page / 2;
            let mut next_focus = current_focus_no + max_row_count;

            next_focus = next_focus / max_row_count * max_row_count;

            // 全体のファイル数を超えないようにする
            if next_focus < app.file_list().len() {
                app.set_focus_file_no(next_focus);
            }
            Ok(AppEvent::PROCESSED)
        }
        KeyCode::Left => {
            // 1画面2分割されてるので、右側にフォーカスがある場合は、行数を引いて、左横に移動する
            // 左横に移動した場合は、先頭にフォーカスを当てる
            let max_row_count = app.max_files_per_page() / 2;
            let mut next_focus: usize;

            // これ以上左に進めない場合はなにもしない
            let current_focus_no = app.focus_file_no();
            if current_focus_no >= max_row_count {
                next_focus = current_focus_no - max_row_count;
                next_focus = next_focus / max_row_count * max_row_count;
                app.set_focus_file_no(next_focus);
            }
            Ok(AppEvent::PROCESSED)
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if let PathKind::DIR(dir) = app.focused_file() {
                let new_dir_path = if dir.file_name == ".." {
                    Path::new(&app.dir_path()).parent().unwrap().to_path_buf()
                } else {
                    Path::new(&app.dir_path()).join(&dir.file_name)
                };
                let new_dir_path = new_dir_path.to_str().unwrap();
                if let Err(e) = app.set_dir_path(&new_dir_path) {
                    app.set_error_message(&e.to_string());
                }
            }
            Ok(AppEvent::PROCESSED)
        }

        _ => {
            Ok(AppEvent::NONE)
        }
    }
}
