use derive_new::new;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

#[derive(new)]
pub struct Input<'a> {
    text: &'a str,
}

impl Widget for Input<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Message (press Enter to send)")
            .style(Style::default().fg(Color::White));

        let inner = block.inner(area);
        block.render(area, buf);

        let paragraph = Paragraph::new(self.text).wrap(Wrap { trim: false });
        paragraph.render(inner, buf);
    }
}
