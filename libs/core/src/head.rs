use crate::accidental::Accidental;

#[derive(Debug, Clone, PartialEq)]
pub struct HeadItem {
    pub level: i8,
    pub accidental: Accidental,
}

#[derive(Debug)]
pub enum HeadType {
    Black,
    White,
    Whole,
    Brevis,
}

#[derive(Debug)]
pub enum HeadVariant {
    Hidden,
    Normal,
    Slash,
    X,
}

pub enum RestType {
    Brevis,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}
