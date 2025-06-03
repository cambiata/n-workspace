use std::{cell::RefCell, collections::BTreeMap};

use crate::{complex::Complex, note::NoteItem, part::PartItem, sysitem::SysItem, voice::stemitems::StemItem, ItemId};

#[derive(Debug)]
pub struct CoreContext {
    pub notes: RefCell<Vec<NoteItem>>,
    pub map_noteid_complexid: RefCell<BTreeMap<ItemId, ItemId>>,
    pub parts: RefCell<Vec<PartItem>>,
    pub sysitems: RefCell<Vec<SysItem>>,
    pub complexes: RefCell<Vec<Complex>>,
    pub stemitems: RefCell<Vec<StemItem>>,
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
        };
        Box::leak(Box::new(cx))
    }
}
