use derive_new::new;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, List, ListItem, Widget},
};

use crate::app::message::{Message, MsgFrom};

#[derive(new)]
pub struct ChatHistory<'a> {
    messages: &'a [Message],
}

impl Widget for ChatHistory<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().title(" Chat ");

        let items: Vec<ListItem> = if self.messages.is_empty() {
            vec![ListItem::new(" No messages yet ").style(Style::default().fg(Color::DarkGray))]
        } else {
            self.messages
                .iter()
                .map(|msg| match msg.from {
                    MsgFrom::User => ListItem::new(format!("> You: {}", msg.text))
                        .style(Style::default().fg(Color::LightBlue)),
                    MsgFrom::Ai => ListItem::new(format!("> AI: {}", msg.text))
                        .style(Style::default().fg(Color::LightGreen)),
                })
                .collect()
        };

        List::new(items).block(block).render(area, buf);
    }
}
