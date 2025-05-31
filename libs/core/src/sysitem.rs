use crate::{clef::ClefSignature, duration::SumDuration, part::complex::ComplexInfo, ItemId};

#[derive(Debug)]
pub enum SysItemType {
    Parts(Vec<ItemId>, SumDuration, Vec<Vec<ComplexInfo>>, Vec<(usize, usize)>),
    Clefs(Vec<ClefSignature>),
    Barline,
    Other,
}

#[derive(Debug)]
pub struct SysItem {
    pub id: ItemId,
    pub stype: SysItemType,
}
