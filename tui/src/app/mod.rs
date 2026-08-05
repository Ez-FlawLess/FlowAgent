use color_eyre::eyre::{Context, bail};
use crossterm::event::EventStream;
use getset::Getters;
use ratatui::{DefaultTerminal, Frame};
use ratatui_textarea::TextArea;

use crate::{
    app::message::{Message, MsgFrom},
    ui::input::Input,
};

mod events;
pub mod message;

#[derive(Getters)]
pub struct App {
    exit: bool,
    event_stream: EventStream,
    #[getset(get = "pub")]
    input_txtarea: TextArea<'static>,
    #[getset(get = "pub")]
    messages: Vec<Message>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            event_stream: EventStream::new(),
            input_txtarea: Input::textarea(),
            messages: Vec::new(),
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

    fn submit_user_msg(&mut self) -> color_eyre::Result<()> {
        if self.input_txtarea.is_empty() {
            bail!("input is empty");
        }
        let msg = self.input_txtarea.lines().join("\n");
        self.input_txtarea.clear();

        let msg = Message::new(msg, MsgFrom::User);

        self.messages.push(msg);

        Ok(())
    }
}
