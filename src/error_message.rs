use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Line, widgets::{Block, Borders, Clear, Paragraph, Wrap}, Frame};

use crate::app::App;

pub fn popup_error_message(app: &App, frame: &mut Frame) {
    let error_message = app.error_message();
    if let Some(message) = &*error_message {
        let footer_text = Line::from("なにかキーを押して下さい....").style(Style::default().fg(Color::Red)).centered();
        let block = Block::default()
            .title(" エラー ")
        .title_bottom(footer_text)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red).bg(Color::Black));

        let message_text = Paragraph::new(message.as_str())
            .block(block)
            .wrap(Wrap { trim: true });

        let popup_layout = centered_rect(60, 20, frame.area());

        frame.render_widget(Clear, popup_layout);
        frame.render_widget(message_text, popup_layout);
    }
}

// 中心にポップアップを表示するためのヘルパー関数
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
