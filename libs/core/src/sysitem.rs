use std::collections::BTreeMap;

use crate::{barline::BarlineType, clef::ClefSignature, complex::ComplexInfo, duration::SumDuration, ItemId};

type MapPositionComplexInfo = BTreeMap<usize, ComplexInfo>;
type MapPositionsDurations = BTreeMap<usize, usize>;

#[derive(Debug)]
pub enum SysItemType {
    Parts(Vec<ItemId>, usize, SumDuration, Vec<MapPositionComplexInfo>, MapPositionsDurations),
    Clefs(Vec<ClefSignature>),
    Barline(BarlineType),
    Other,
}

#[derive(Debug)]
pub struct SysItem {
    pub id: ItemId,
    pub stype: SysItemType,
    pub parts_count: usize,
    pub position: usize,
}

impl SysItem {
    pub fn is_parts(&self) -> bool {
        matches!(self.stype, SysItemType::Parts(_, _, _, _, _))
    }
}

//------------------------------------------------------------------------

#[derive(Debug)]
pub struct SysItemList {
    pub sysitem_ids: Vec<usize>,
    // pub parts_items_ids: Vec<usize>,
    pub partscount: usize,
}
