use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect}, 
    style::{Color, Style, Stylize}, symbols::border, text::{Span, Text}, 
    widgets::{block::Title, Block, Borders, List, ListItem, Paragraph}, 
    Frame
};

use crate::{app::App, fms::PathKind};

/// UIを描画する
pub fn ui(frame: &mut Frame, app: &App) {
    ///////////////////////////////
    // フレームの初期化

    // フレームを3つに分割
    let chunks = Layout::default()
    .direction(Direction::Vertical) 
    .constraints([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(frame.area());

    ///////////////////////////////
    // 現在のディレクトリ
    let current_dir_title = Title::from(" Current Directory ".bold());

    let current_dir_block = Block::default()
        .title(current_dir_title.alignment(Alignment::Center))
        .borders(Borders::ALL)
        //.style(Style::default().fg(Color::LightYellow).bg(Color::Black));
        .style(Style::default());

    // let current_dir_path = Paragraph::new(Text::styled(
    //     app.path(), 
    //     Style::default().fg(Color::Green),))
    //     .block(current_dir_block);

    // frame.render_widget(current_dir_path, chunks[0]);

    ///////////////////////////////
    // ファイルリスト
    // let items:Vec<ListItem> = app.file_names()
    // .iter()
    // .map(|file| {
    //   let file_name = match file {
    //     PathKind::FILE(name) => format!("📄 {}", name),
    //     PathKind::DIR(name) => format!("📁 {}", name),
    //   };
    //   ListItem::new(Span::raw(file_name))
    // })
    // .collect();

    // let file_list = List::new(items);
    // frame.render_widget(file_list, chunks[1]);

}