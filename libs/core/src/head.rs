use crate::accidental::Accidental;

#[derive(Debug, Clone, PartialEq)]
pub struct HeadItem {
    pub level: i8,
    pub accidental: Accidental,
}
