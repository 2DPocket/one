
use std::{error::Error, io, path::Path};

use fms::PathKind;
use ratatui::{
    backend::{Backend, CrosstermBackend}, crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    }, layout::Alignment, style::{Color, Style}, Terminal
};


mod app;
use crate::app::App;

mod header;
use crate::header::HeaderWidget;

mod body;

mod footer;
use crate::footer::FooterWidget;

mod ui;
use crate::ui::ui;

mod fms;

mod utils;

fn main() -> Result<(), Box<dyn Error>> {

    ///////////////////////////////
    // Application pre-run steps

    // Initialize the terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    ///////////////////////////////
    // Run the app
    // let mut app = App::new(".")?;
    let mut app = App::new("c:/xampp/php")?;
    let res = run_app(&mut terminal, &app);

    ///////////////////////////////
    // Application post-run steps
    // (boilerplate code)

    // restore terminal state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    terminal.show_cursor()?;

    Ok(())

}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let area = f.area();
            const MIN_WITH: u16 = 90;
            const MIN_HEIGHT: u16 = 20;

            if area.width < MIN_WITH || area.height < MIN_HEIGHT {
                let warning = ratatui::widgets::Paragraph::new(
                    format!("表示可能なターミナルの最小サイズは(W x H)は{}x{}です。", MIN_WITH, MIN_HEIGHT))
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center);
                
                f.render_widget(warning, area);                
                return;
            }

            app.draw(f)
        })?;

        // Event handling

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::F(1) => {
                    return Ok(());
                }
                KeyCode::Down => {
                    let current_focus_no = app.focus_file_no();
                    if current_focus_no < app.file_list().len() - 1 {
                        app.set_focus_file_no(current_focus_no + 1);
                    }
                }
                KeyCode::Up => {
                    let current_focus_no = app.focus_file_no();
                    if current_focus_no > 0 {
                        app.set_focus_file_no(current_focus_no - 1);
                    }
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
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if let PathKind::DIR(dir) = app.focused_file() {
                        let new_dir_path = if dir.file_name == ".." {
                            Path::new(&app.dir_path()).parent().unwrap().to_path_buf()
                        } else {
                            Path::new(&app.dir_path()).join(&dir.file_name)
                        };
                        let new_dir_path = new_dir_path.to_str().unwrap();
                        app.set_dir_path(&new_dir_path);
                    }
                }

                _ => {}
            }
        }

    }    
}
