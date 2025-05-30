use crate::{duration::NoteDuration, head::HeadItem, ItemId};

#[derive(Debug, PartialEq, Clone)]
pub enum NoteType {
    Heads(Vec<HeadItem>),
    Rest,
    LyricItem,
}

#[derive(Debug, Clone)]
pub struct NoteItem {
    pub id: ItemId,
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
}
