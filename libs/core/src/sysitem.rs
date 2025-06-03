use std::collections::BTreeMap;

use crate::{barline::BarlineType, clef::ClefSignature, duration::SumDuration, part::complex::ComplexInfo, ItemId};

#[derive(Debug)]
pub enum SysItemType {
    Parts(Vec<ItemId>, SumDuration, Vec<BTreeMap<usize, ComplexInfo>>, BTreeMap<usize, usize>),
    Clefs(Vec<ClefSignature>),
    Barline(BarlineType),
    Other,
}

#[derive(Debug)]
pub struct SysItem {
    pub id: ItemId,
    pub stype: SysItemType,
    pub parts_count: usize,
}
