use color_eyre::eyre::Context;
use crossterm::event::EventStream;
use ratatui::{DefaultTerminal, Frame};

mod events;

pub struct App {
    exit: bool,
    event_stream: EventStream,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_stream: EventStream::new(),
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            terminal
                .draw(|frame| self.render_frame(frame))
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
