use crate::accidental::Accidental;

pub type HeadId = usize;

#[derive(Debug, Clone)]
pub struct HeadItem {
    pub id: HeadId,
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
