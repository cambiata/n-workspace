use std::{cell::RefCell, collections::BTreeMap};

use crate::{
    complex::Complex,
    head::HeadItem,
    note::NoteItem,
    part::PartItem,
    stems::stemitems::{StemHeadPosition, StemItem},
    sysitem::SysItem,
    ItemId,
};

#[derive(Debug)]
pub struct CoreContext {
    pub heads: RefCell<Vec<HeadItem>>,
    pub notes: RefCell<Vec<NoteItem>>,
    pub map_noteid_complexid: RefCell<BTreeMap<ItemId, ItemId>>,
    pub parts: RefCell<Vec<PartItem>>,
    pub sysitems: RefCell<Vec<SysItem>>,
    pub complexes: RefCell<Vec<Complex>>,
    pub stemitems: RefCell<Vec<StemItem>>,
    pub map_head_position: RefCell<BTreeMap<usize, StemHeadPosition>>,
}

impl CoreContext {
    pub fn new() -> &'static CoreContext {
        let cx = CoreContext {
            heads: RefCell::new(Vec::new()),
            notes: RefCell::new(Vec::new()),
            map_noteid_complexid: RefCell::new(BTreeMap::new()),
            parts: RefCell::new(Vec::new()),
            sysitems: RefCell::new(Vec::new()),
            complexes: RefCell::new(Vec::new()),
            stemitems: RefCell::new(Vec::new()),
            map_head_position: RefCell::new(BTreeMap::new()),
        };
        Box::leak(Box::new(cx))
    }
}
