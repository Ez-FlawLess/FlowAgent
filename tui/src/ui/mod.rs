use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::{
    app::App,
    ui::{chat_history::ChatHistory, input::Input, status::Status},
};

mod chat_history;
pub mod input;
mod status;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [chat_his_area, input_area, status_area] = Layout::vertical([
            Constraint::Min(3),
            Constraint::Length(4),
            Constraint::Length(1),
        ])
        .areas(area);

        ChatHistory.render(chat_his_area, buf);
        Input::new(self.input_txtarea()).render(input_area, buf);
        Status.render(status_area, buf);
    }
}
