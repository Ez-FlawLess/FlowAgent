use color_eyre::eyre::Context;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
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
            KeyCode::Esc => self.exit(),
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::Enter if key_event.modifiers.contains(KeyModifiers::SHIFT) => {
                self.input_txtarea.insert_newline();
            }
            KeyCode::Enter => {
                let _ = self.submit_user_msg();
            }
            _ => {
                self.input_txtarea.input(key_event);
            }
        }
    }
}
