pub mod utils;

use std::{cell::RefCell, collections::BTreeMap};

use crate::{
    note::NoteItem,
    part::{complex::ComplexItem, PartItem},
    sysitem::SysItem,
    voice::stemitems::StemItem,
    ItemId,
};

#[derive(Debug)]
pub struct CoreContext {
    pub notes: RefCell<Vec<NoteItem>>,
    pub map_noteid_complexid: RefCell<BTreeMap<ItemId, ItemId>>,
    pub parts: RefCell<Vec<PartItem>>,
    pub sysitems: RefCell<Vec<SysItem>>,
    pub complexes: RefCell<Vec<ComplexItem>>,
    pub stemitems: RefCell<Vec<StemItem>>,
    // pub map_partid_complexids: RefCell<BTreeMap<ItemId, Vec<ItemId>>>,
    // pub map_sysitemid_complexdurations: RefCell<BTreeSet<ItemId>>,
}

impl CoreContext {
    pub fn new() -> &'static CoreContext {
        let cx = CoreContext {
            notes: RefCell::new(Vec::new()),
            map_noteid_complexid: RefCell::new(BTreeMap::new()),
            parts: RefCell::new(Vec::new()),
            sysitems: RefCell::new(Vec::new()),
            complexes: RefCell::new(Vec::new()),
            stemitems: RefCell::new(Vec::new()),
            // map_partid_complexids: RefCell::new(BTreeMap::new()),
            // map_sysitemid_complexdurations: RefCell::new(BTreeSet::new()),
        };
        Box::leak(Box::new(cx))
    }
}
