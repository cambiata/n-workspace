use crate::color::Color;

#[derive(Debug, Clone)]
pub enum Fill {
    None,
    Solid(Color),
}
