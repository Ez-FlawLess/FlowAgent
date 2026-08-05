use color_eyre::eyre::Context;
use crossterm::event::EventStream;
use getset::Getters;
use ratatui::{DefaultTerminal, Frame};
use ratatui_textarea::TextArea;

use crate::ui::input::Input;

mod events;

#[derive(Getters)]
pub struct App {
    exit: bool,
    event_stream: EventStream,
    #[getset(get = "pub")]
    input_txtarea: TextArea<'static>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_stream: EventStream::new(),
            input_txtarea: Input::textarea(),
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            terminal
                .draw(|frame| {
                    self.render_frame(frame);
                })
                .wrap_err("failed to draw frame")?;
            self.handle_event()
                .await
                .wrap_err("failed to handle events")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
