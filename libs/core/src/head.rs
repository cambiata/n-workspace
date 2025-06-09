use crate::{accidental::Accidental, stems::stemitems::StemHeadPosition};

#[derive(Debug, Clone)]
pub struct HeadItem {
    pub level: i8,
    pub accidental: Accidental,
    pub head_position: Option<StemHeadPosition>,
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
