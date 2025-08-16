use std::time::Duration;

use iced::{
    alignment::Horizontal::Left,
    widget::{center, column, svg, text, text_input, Svg, Text, TextInput},
    Element, Task,
};

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
                self.svg_string = match app_svg::generate(&self.debounced_text) {
                    Ok(svg) => svg,
                    Err(err) => {
                        eprintln!("Error generating SVG: {}", err);
                        String::from("0 1 2")
                    }
                };
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, AppMessage> {
        let input: TextInput<'_, AppMessage> = text_input("", self.input_text.as_str()).on_input(AppMessage::InputChanged).padding(15).size(30).align_x(Left);
        let text: Text = text(self.debounced_text.as_str()).size(30);

        let svg: Svg = svg(svg::Handle::from_memory(self.svg_string.clone().into_bytes()));
        center(column![input, text, svg]).into()
    }

    fn new() -> (Self, Task<AppMessage>) {
        (
            Self {
                input_text: "clef G | 0,1".to_string(),
                debounced_text: "clef G | 0,1".to_string(),
                abort_handler: None,
                svg_string: SVG_BLUE.to_string(),
            },
            Task::perform(tokio::time::sleep(Duration::from_millis(0)), |_| AppMessage::SetDebouncedValue),
        )
    }
}

const SVG_BLUE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><circle cx="50" cy="50" r="40" fill="blue"/></svg>"#;

mod app_svg {

    use core::context::CoreContext;
    use graphics::graphicitem::GraphicItems;
    use grid::{gridcontext::GridContext, griditem::GridItemType};
    use parse::parse2::Parse2;
    use render::gridrender::Render;
    use score::{build::BuildScore, glyphitem::GlyphItem, scorecontext::ScoreContext};
    use svg::builder::SvgBuilder;

    pub fn generate(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder for score generation logic
        let cx = CoreContext::new();
        // let _ = Parse2::sysitemlist2(cx, "clef G F | D8 -3 n-1 #4 3 r -2 -2 -3 / 0 ", false).unwrap();
        let _ = Parse2::sysitemlist2(cx, input, false).unwrap();

        // let _ = Parse2::sysitemlist2(cx, "|clef G |0 -1 -2 -3 -4 -5 -6 -7  5 4 3 2 1 0 -1 -2 -3 -4 -5 % 1 1 2 3 4 5 6 7 ", false).unwrap();

        // dbg!(&cx.stemitems.borrow());

        let scx = ScoreContext::new();
        BuildScore::build(&scx, &cx)?;

        //-------------------------------------------------
        // Turn 180 degrees...
        let items = scx.grid_columns.borrow().to_vec();
        let mut items2: Vec<Vec<GridItemType<GlyphItem>>> = Vec::new();
        let rows = items[0].len();
        for row in 0..rows {
            let mut rowitems = Vec::new();
            for col in 0..items.len() {
                rowitems.push(items[col][row].clone());
            }
            items2.push(rowitems);
        }
        //-------------------------------------------------
        let gcx = GridContext::<GlyphItem>::new();
        gcx.add_items(items2)?;

        // calculate distances
        let allotments: Vec<f32> = scx.grid_column_allotment.borrow().to_vec();
        gcx.handle_column_spacing(&allotments, 2.3)?;
        gcx.handle_row_heights()?;

        // create graphic items
        let mut graphic_items = GraphicItems::new();
        let notelines = Render::render_notelines(&gcx);
        graphic_items.extend(notelines);
        let glyphitems = Render::render_music_glyphitems(&gcx);
        graphic_items.extend(glyphitems);

        // save to svg
        let svg_string = SvgBuilder::new().build(graphic_items, None);
        Ok(svg_string)
    }
}
