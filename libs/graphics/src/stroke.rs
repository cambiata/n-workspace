use crate::color::Color;

#[derive(Debug, Clone)]
pub enum Stroke {
    None,
    Solid(f32, Color),
}
