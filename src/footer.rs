use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Block, Paragraph, Widget}
};

pub struct FooterWidget;

impl Widget for FooterWidget {
    fn render (self, area: Rect, buf: &mut Buffer) {

        let fnc1 = FunctionKeyWidget::new("Quit", "F1");
        fnc1.render(area, buf);
        let fnc2 = FunctionKeyWidget::new("Quit", "F2");
        fnc2.render(area, buf);
    }
}

pub struct FunctionKeyWidget {
    label: String,
    key: String,
}

impl FunctionKeyWidget {
    pub fn new(label: &str, key: &str) -> Self {
        Self {
            label: label.to_string(),
            key: key.to_string(),
        }
    }
}

impl Widget for FunctionKeyWidget {
    fn render (self, area: Rect, buf: &mut Buffer) {
        let mut keys:Vec<(FunctionKeyWidget)> = Vec::new();
        keys.push(FunctionKeyWidget::new("終了", "F1"));
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
                Span::styled(&key.key, Style::default().fg(Color::Yellow).bg(Color::Gray)),
                Span::styled(" ", Style::default().bg(Color::Gray)),
            ]);
            
            key_label.render(chunks[i], buf);
        }    


   }
}