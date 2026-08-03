use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().bg(Color::Gray);

        let inner_area = block.inner(area);

        block.render(area, buf);

        let [centered_area] = Layout::vertical([Constraint::Length(1)])
            .flex(Flex::Center)
            .areas(inner_area);

        Paragraph::new("Hello World")
            .centered()
            .fg(Color::Black)
            .render(centered_area, buf);
    }
}
