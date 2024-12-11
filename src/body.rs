use std::vec;

use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{List, ListItem, Widget}
};

use crate::{app::App, fms::PathKind};

pub struct BodyWidget<'a> {
    app: &'a App,
}

impl BodyWidget<'_> {
    pub fn new<'a>(app: &'a App) -> BodyWidget<'a> {
        BodyWidget { app }
    }
}

impl Widget for BodyWidget<'_> {
    fn render (self, area: Rect, buf: &mut Buffer) {
        // 描画エリアを2分割
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        
        let  area_with = chunks[0].width;

        // ファイルリスト
        let draw_item_max_count = area.height * 2;
        self.app.set_max_files_per_page(draw_item_max_count as usize);

        // 現在のページ番号に基づいてオフセットを計算
        let current_page_no = self.app.current_page_no();
        let offset = (current_page_no - 1) * draw_item_max_count as usize;

        // 現在のページ番号に基づいてファイルリストを描画
        let mut left_items: Vec<ListItem> = Vec::new();
        let mut right_items: Vec<ListItem> = Vec::new();
        let file_list = self.app.file_list();
        for (i, file) in file_list.iter().skip(offset).take(draw_item_max_count as usize).enumerate() {
            // ファイル情報の描画ラインインスタンスを取得
            let file_info_line = get_file_info_line(&self.app, i, offset, file, area_with);   
            if i < area.height as usize {
                left_items.push(file_info_line);
            } else {
                right_items.push(file_info_line);
            }
        }
        render_file_list(left_items, &chunks[0], buf);
        render_file_list(right_items, &chunks[1], buf);

    }

}


/// ファイル情報のListItemを取得
/// 
/// # 引数
/// * `app` - アプリケーション
/// * `index` - ファイルリストのインデックス
/// * `offset` - ファイルリストのオフセット
/// * `file_info` - ファイル情報
/// * `area_with` - 描画エリアの幅
/// 
/// # 戻り値
/// `ListItem` - ファイル情報のListItem
/// 
fn get_file_info_line<'a>(app: &App, index: usize, offset: usize, file_info: &'a PathKind, area_width: u16) -> ListItem<'a> {
    // 背景色
    let bg = if  (index+offset) == app.focus_file_no() {
        Color::LightBlue
    } else {
        Color::Reset
    };

    // 名称の最大長さ
    // 16+2 : 日付(YYYY/MM/DD HH:MM) + スペース
    // 8+2 : サイズ(9,999.9G) + スペース
    // 1 : スペース
    let name_max_len = area_width - ((16+2) + (8+2) + 3) ;
    let file_info_spans = match file_info {
        PathKind::FILE(file_info) => 
            vec!(
                Span::styled(file_info.format_file_name(name_max_len.into()), Style::default().bg(bg)), 
                Span::styled("  ", Style::default().bg(bg)),
                Span::styled(file_info.formatted_modified(), Style::default().bg(bg)), 
                Span::styled("  ", Style::default().bg(bg)),
                Span::styled(file_info.format_file_size(), Style::default().bg(bg))
            ),
        PathKind::DIR(file_info) => 
            vec!(
                Span::styled(file_info.format_file_name(name_max_len.into()), Style::default().fg(Color::Yellow).bg(bg)),
                Span::styled("  ", Style::default().bg(bg)),
                Span::styled(file_info.formatted_modified(), Style::default().bg(bg)),
                Span::styled("  ", Style::default().bg(bg)),
                Span::styled(format!("{:>8}", "<DIR>"), Style::default().fg(Color::Yellow).bg(bg))
            ),
    };
    ListItem::from(Line::from(file_info_spans))   
}



/// ファイルリストを描画する
///
/// # 引数
/// * `list_items` - ファイルリスト
/// * `area` - 描画エリア
/// * `buf` - 描画バッファ
/// 
/// # 戻り値
/// なし
///  
fn render_file_list(list_items: Vec<ListItem>, area: &Rect, buf: &mut Buffer) {
    let file_list = List::new(Vec::from(list_items));
    file_list.render(*area, buf);
}

