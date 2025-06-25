use crate::{
    duration::NoteDuration,
    head::{HeadId, HeadItem},
    ItemId,
};

pub type NoteId = usize;

#[derive(Debug, Clone)]
pub enum NoteType {
    Heads(Vec<HeadItem>),
    // Heads(Vec<usize>, Vec<i8>),
    Rest,
    LyricItem,
}

#[derive(Debug, Clone)]
pub struct NoteItem {
    pub id: NoteId,
    pub position: usize,
    pub duration: NoteDuration,
    pub ntype: NoteType,
}

impl NoteItem {
    pub fn is_beamable(&self) -> bool {
        match self.ntype {
            NoteType::Heads(_) => {
                //
                NoteDuration::is_beamable(self.duration)
            }
            _ => false,
        }
    }
    pub fn has_stem(&self) -> bool {
        match self.ntype {
            NoteType::Heads(_) => {
                //
                true
            }
            _ => false,
        }
    }

    pub fn get_head_levels(&self) -> Option<Vec<i8>> {
        match &self.ntype {
            NoteType::Heads(heads) => Some(heads.iter().map(|h| h.level).collect()),
            _ => None,
        }
    }
    pub fn get_head_ids_and_levels(&self) -> Option<Vec<(HeadId, i8)>> {
        match &self.ntype {
            NoteType::Heads(heads) => Some(heads.iter().map(|h| (h.id, h.level)).collect()),
            _ => None,
        }
    }
}
