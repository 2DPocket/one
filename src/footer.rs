use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::Widget
};


pub struct FunctionKey {
    label: String,
    key: String,
}

impl FunctionKey {
    pub fn new(label: &str, key: &str) -> Self {
        Self {
            label: label.to_string(),
            key: key.to_string(),
        }
    }
}

pub struct FooterWidget;

impl Widget for FooterWidget {
    fn render (self, area: Rect, buf: &mut Buffer) {
        let mut keys:Vec<FunctionKey> = Vec::new();
        keys.push(FunctionKey::new("終了", "q"));
        // keys.push(FunctionKeyWidget::new("XXXX", "F2"));
        
        const KEY_LABEL_LENGTH: u16 = 12;

        let chunks =   Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(KEY_LABEL_LENGTH); keys.len()
        ])
        .split(area);  //keys.push(FunctionKeyWidget::new("Save", "F2"));

        for (i, key) in keys.iter().enumerate() {
            let key_label = Line::from(vec![
                Span::styled(" ", Style::default().bg(Color::Gray)),
                Span::styled(&key.label, Style::default().fg(Color::Black).bg(Color::Gray)),
                Span::styled(" ", Style::default().bg(Color::Gray)),
                Span::styled(format!("<{}>", &key.key), Style::default().fg(Color::LightBlue).bg(Color::Gray)),
                Span::styled(" ", Style::default().bg(Color::Gray)),
            ]);
            
            key_label.render(chunks[i], buf);
        }    


   }
}