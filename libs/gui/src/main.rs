use std::{fmt::Error, time::Duration};

use iced::{
    alignment::Horizontal::Left,
    widget::{center, column, svg, text, text_input, Svg, Text, TextInput},
    Element, Task,
};
use render::output::Generate;

pub fn main() -> iced::Result {
    iced::application("My App", App::update, App::view).run_with(App::new)
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    InputChanged(String),
    SetDebouncedValue,
}

#[derive(Debug, Default)]
struct App {
    input_text: String,
    debounced_text: String,
    abort_handler: Option<iced::task::Handle>,
    svg_string: String,
    error_message: String,
}

impl App {
    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            // Called whenever text input changes
            AppMessage::InputChanged(value) => {
                self.input_text = value;

                // If abort_handler is set, abort it
                self.abort_handler.as_ref().map(|abort_handler| abort_handler.abort());

                // Create a handler for the async task
                let handler = Task::perform(tokio::time::sleep(Duration::from_millis(1000)), |_| AppMessage::SetDebouncedValue);

                // Split handler into task_handler and abort_handler
                let (task_handler, abort_handler) = handler.abortable();

                // store the abort_handler
                self.abort_handler = Some(abort_handler);

                return task_handler;
            }

            AppMessage::SetDebouncedValue => {
                self.debounced_text = self.input_text.clone();
                match Generate::svg_string(&self.debounced_text) {
                    Ok(svg) => {
                        self.svg_string = svg;
                        self.error_message.clear();
                    }
                    Err(err) => {
                        eprintln!("Error generating SVG: {}", &err);
                        self.error_message = err.to_string();
                    }
                };
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, AppMessage> {
        let error: Text = text(self.error_message.as_str()).size(20).color(iced::Color::from_rgb(1.0, 0.0, 0.0));
        let input: TextInput<'_, AppMessage> = text_input("", self.input_text.as_str()).on_input(AppMessage::InputChanged).padding(15).size(30).align_x(Left);
        let text: Text = text(self.debounced_text.as_str()).size(30);

        let svg: Svg = svg(svg::Handle::from_memory(self.svg_string.clone().into_bytes()));
        column![error, input, /*text,*/ svg].into()
    }

    fn new() -> (Self, Task<AppMessage>) {
        (
            Self {
                input_text: "clef G | 0,1".to_string(),
                debounced_text: "clef G | 0,1".to_string(),
                abort_handler: None,
                svg_string: SVG_BLUE.to_string(),
                error_message: String::new(),
            },
            Task::perform(tokio::time::sleep(Duration::from_millis(0)), |_| AppMessage::SetDebouncedValue),
        )
    }
}

const SVG_BLUE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><circle cx="50" cy="50" r="40" fill="blue"/></svg>"#;

// mod app_svg {
//     use render::output::Generate;

//     pub fn generate(code: &str) -> Result<String, Box<dyn std::error::Error>> {
//         Generate::svg_string(code)
//     }
// }
