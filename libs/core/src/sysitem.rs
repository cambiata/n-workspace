use std::collections::BTreeMap;

use crate::{barline::BarlineType, clef::ClefSignature, complex::ComplexInfo, duration::SumDuration, part::PartId};

pub type SysitemId = usize;
pub type SysitemPosition = usize;
pub type PartsCount = usize;

type MapPositionComplexInfo = BTreeMap<usize, ComplexInfo>;
type MapPositionsDurations = BTreeMap<usize, usize>;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
pub enum SysItemTypeId {
    Clefs(Vec<String>),
    Parts(Vec<Vec<String>>),
    Barlines(Vec<String>),
}

#[derive(Debug)]
pub enum SysItemType {
    Parts(Vec<PartId>, SumDuration, Vec<MapPositionComplexInfo>, MapPositionsDurations),
    Clefs(Vec<ClefSignature>),
    Barline(BarlineType),
    Other,
}

#[derive(Debug)]
pub struct SysItem {
    pub id: SysitemId,
    pub stype: SysItemType,
    pub parts_count: PartsCount,
    pub position: usize,
    pub duration: usize,
}

impl SysItem {
    pub fn is_parts(&self) -> bool {
        matches!(self.stype, SysItemType::Parts(_, _, _, _))
    }
}

pub type VecPartNotes = Vec<(Option<usize>, SysitemPosition, SysitemId)>;

//------------------------------------------------------------------------

#[derive(Debug)]
pub struct SysItemList {
    pub sysitem_ids: Vec<usize>,
    // pub parts_items_ids: Vec<usize>,
    pub partscount: usize,
    pub partsnotesvecs: Vec<(VecPartNotes, VecPartNotes)>,
}
