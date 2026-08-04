use color_eyre::eyre::Context;
use crossterm::event::EventStream;
use getset::Getters;
use ratatui::{DefaultTerminal, Frame};

mod events;

#[derive(Getters)]
pub struct App {
    exit: bool,
    event_stream: EventStream,
    #[getset(get = "pub")]
    input: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_stream: EventStream::new(),
            input: String::new(),
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

    fn add_to_input(&mut self, char: char) {
        self.input.push(char);
    }

    fn delete_from_input(&mut self) {
        let _ = self.input.pop();
    }
}
