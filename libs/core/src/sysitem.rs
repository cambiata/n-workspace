use crate::{clef::ClefSignature, duration::SumDuration, ItemId};

#[derive(Debug)]
pub enum SysItemType {
    Parts(Vec<ItemId>, SumDuration),
    Clefs(Vec<ClefSignature>),
    Barline,
    Other,
}

#[derive(Debug)]
pub struct SysItem {
    pub id: ItemId,
    pub stype: SysItemType,
}
