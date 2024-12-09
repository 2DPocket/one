use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, symbols::border, text::{Line, Span}, widgets::{Block, Paragraph, Widget}
};

use crate::{app::App, utils::div_cell};

pub struct HeaderWidget<'a> {
    app: &'a App,
}

impl HeaderWidget<'_> {
    pub fn new<'a>(app: &'a App) -> HeaderWidget<'a> {
        HeaderWidget { app }
    }
}

impl Widget for HeaderWidget<'_> {
    fn render (self, area: Rect, buf: &mut Buffer) {

        // タイトルの設定    
        let title = Line::from(" -ONE- file and directory manager ").style(Style::default().fg(Color::Blue));

        // ブロックの設定
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(Color::Blue));

        // 現在のディレクトリパス    
        let dir_path_label = "PATH=";
        let dir_path_value = self.app.dir_path();

        // ページ情報（現在のページ数/全ページ数）
        let page_label = "PAGE=";
        let total_pages = div_cell(self.app.count_file_list(),self.app.max_files_per_page());
        let page_value = format!("{}/{}", self.app.current_page_no(), total_pages);


        // `block` の内部の幅を計算(両端の枠線分(2)を除く)
        let inner_width = area.width.saturating_sub(2) as usize;

        let dir_path_len = dir_path_label.len() + dir_path_value.len();
        let page_len = page_label.len() + page_value.len();
        let space_between = inner_width.saturating_sub(dir_path_len + page_len);

        // 現在のディレクトリパスとページ情報をフォーマット
        let formatted_line = Line::from(vec![
            Span::styled(dir_path_label, Style::default().fg(Color::LightCyan)),
            Span::styled(dir_path_value, Style::default().fg(Color::Blue)),
            Span::raw(" ".repeat(space_between)),
            Span::styled(page_label, Style::default().fg(Color::LightCyan)),
            Span::styled(page_value, Style::default().fg(Color::Blue)),
        ]);

        Paragraph::new(formatted_line)
            .block(block)
            .alignment(Alignment::Left)
            .render(area, buf);

    }
}