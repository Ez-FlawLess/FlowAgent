use derive_new::new;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};
use ratatui_textarea::{TextArea, WrapMode};

#[derive(new)]
pub struct Input<'a> {
    textarea: &'a TextArea<'a>,
}

impl<'a> Input<'a> {
    pub fn textarea() -> TextArea<'a> {
        let mut text_area = TextArea::default();
        text_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Message (press Enter to send)")
                .style(Style::default().fg(Color::White)),
        );
        text_area.set_wrap_mode(WrapMode::Word);

        text_area
    }
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.textarea.render(area, buf);
    }
}
