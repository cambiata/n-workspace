use crate::accidental::Accidental;

#[derive(Debug, Clone, PartialEq)]
pub struct HeadItem {
    pub level: i8,
    pub accidental: Accidental,
}

#[derive(Debug, Clone)]
pub enum HeadType {
    Black,
    White,
    Whole,
    Brevis,
}

#[derive(Debug, Clone)]
pub enum HeadVariant {
    Hidden,
    Normal,
    Slash,
    X,
}
