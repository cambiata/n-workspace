use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Gray,
    LightGray,
    White,
    Red,
    Blue,
    Green,
    Tomato,
    DodgerBlue,
    Orange,
    Yellow,
    Lime,
    Purple,
    RGBA(u8, u8, u8, f32),
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "black"),
            Color::LightGray => write!(f, "lightgray"),
            Color::Gray => write!(f, "gray"),
            Color::White => write!(f, "white"),
            Color::Red => write!(f, "red"),
            Color::Blue => write!(f, "blue"),
            Color::Green => write!(f, "green"),
            Color::Tomato => write!(f, "tomato"),
            Color::DodgerBlue => write!(f, "dodgerblue"),
            Color::Orange => write!(f, "orange"),
            Color::Yellow => write!(f, "yellow"),
            Color::Lime => write!(f, "lime"),
            Color::Purple => write!(f, "purple"),
            Color::RGBA(r, g, b, a) => {
                // Custom color in rgba format
                return write!(f, "rgba({},{},{},{})", r, g, b, a);
            }
        }
    }
}
