use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

pub struct Status;

impl Widget for Status {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new("press Esc / Ctrl+c to quit")
            .style(Style::default().fg(Color::DarkGray));

        paragraph.render(area, buf);
    }
}
