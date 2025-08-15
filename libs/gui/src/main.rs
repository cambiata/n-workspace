use std::time::Duration;

use iced::{
    alignment::Horizontal::Left,
    widget::{center, column, text, text_input, Text, TextInput},
    Element, Task,
};

pub fn main() -> iced::Result {
    iced::application("My App", Test::update, Test::view).run_with(Test::new)
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SetDebouncedValue,
}

#[derive(Debug, Default)]
struct Test {
    input_text: String,
    debounced_text: String,
    abort_handler: Option<iced::task::Handle>,
}

impl Test {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Called whenever text input changes
            Message::InputChanged(value) => {
                self.input_text = value;

                // If abort_handler is set, abort it
                self.abort_handler.as_ref().map(|abort_handler| abort_handler.abort());

                // Create a handler for the async task
                let handler = Task::perform(tokio::time::sleep(Duration::from_millis(800)), |_| Message::SetDebouncedValue);

                // Split handler into task_handler and abort_handler
                let (task_handler, abort_handler) = handler.abortable();

                // store the abort_handler
                self.abort_handler = Some(abort_handler);

                return task_handler;
            }

            Message::SetDebouncedValue => {
                self.debounced_text = self.input_text.clone();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let input: TextInput<'_, Message> = text_input("", self.input_text.as_str()).on_input(Message::InputChanged).padding(15).size(30).align_x(Left);
        let text: Text = text(self.debounced_text.as_str()).size(30);
        center(column![input, text]).into()
    }

    fn new() -> (Self, Task<Message>) {
        (
            Self {
                input_text: "Input".to_string(),
                debounced_text: "Debounced".to_string(),
                abort_handler: None,
            },
            Task::none(),
        )
    }
}
