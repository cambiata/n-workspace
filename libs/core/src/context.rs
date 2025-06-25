use std::{cell::RefCell, collections::BTreeMap};

use crate::{
    complex::{Complex, ComplexId},
    head::{HeadId, HeadItem},
    note::{NoteId, NoteItem},
    part::PartItem,
    stems::stemitems::{StemHeadPosition, StemItem},
    sysitem::SysItem,
    ties::{ResolvedTieFrom, ResolvedTieTo, TieFrom, TieTo},
};

#[derive(Debug)]
pub struct CoreContext {
    pub heads: RefCell<Vec<HeadItem>>,
    pub notes: RefCell<Vec<NoteItem>>,
    pub map_noteid_complexid: RefCell<BTreeMap<NoteId, ComplexId>>,
    pub parts: RefCell<Vec<PartItem>>,
    pub sysitems: RefCell<Vec<SysItem>>,
    pub complexes: RefCell<Vec<Complex>>,
    pub stemitems: RefCell<Vec<StemItem>>,
    pub map_head_position: RefCell<BTreeMap<HeadId, StemHeadPosition>>,
    pub map_noteid_tiesto: RefCell<BTreeMap<NoteId, Vec<TieTo>>>,
    pub map_noteid_tiesfrom: RefCell<BTreeMap<NoteId, Vec<TieFrom>>>,
    pub map_noteid_resolvedtiesto: RefCell<BTreeMap<NoteId, Vec<ResolvedTieTo>>>,
    pub map_noteid_resolvedtiesfrom: RefCell<BTreeMap<NoteId, Vec<ResolvedTieFrom>>>,
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
            map_noteid_tiesto: RefCell::new(BTreeMap::new()),
            map_noteid_tiesfrom: RefCell::new(BTreeMap::new()),
            map_noteid_resolvedtiesto: RefCell::new(BTreeMap::new()),
            map_noteid_resolvedtiesfrom: RefCell::new(BTreeMap::new()),
        };
        Box::leak(Box::new(cx))
    }
}
