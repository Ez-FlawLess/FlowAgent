use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use futures_util::StreamExt;
use thiserror::Error;

use crate::app::App;

impl App {
    pub async fn handle_event(&mut self) -> Result<(), HandleEventsErr> {
        if let Some(event) = self.event_stream.next().await {
            match event? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
    }
}

#[derive(Error, Debug)]
pub enum HandleEventsErr {
    #[error("io error reading event: {0}")]
    Io(#[from] io::Error),
}
