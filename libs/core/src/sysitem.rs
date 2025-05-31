use crate::{clef::ClefSignature, duration::SumDuration, part::complex::ComplexInfo, ItemId};

#[derive(Debug)]
pub enum SysItemType {
    Parts(Vec<ItemId>, SumDuration, Vec<Vec<ComplexInfo>>),
    Clefs(Vec<ClefSignature>),
    Barline,
    Other,
}

#[derive(Debug)]
pub struct SysItem {
    pub id: ItemId,
    pub stype: SysItemType,
    // pub complexes_durations: Vec<usize>,
}
