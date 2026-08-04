use color_eyre::eyre::Context;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use futures_util::StreamExt;

use crate::app::App;

impl App {
    pub async fn handle_event(&mut self) -> color_eyre::Result<()> {
        let Some(event) = self.event_stream.next().await else {
            return Ok(());
        };

        match event.wrap_err("io error reading event")? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
    }
}
