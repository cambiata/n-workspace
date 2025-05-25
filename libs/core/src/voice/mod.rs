pub mod stemdirections;
pub mod stemitems;

use crate::duration::SumDuration;

#[derive(Debug)]
pub enum VoiceType {
    Barpause,
    NoteIds(Vec<usize>, SumDuration, Vec<usize>),
}

#[derive(Debug)]
pub struct VoiceItem {
    pub duration: SumDuration,
    pub vtype: VoiceType,
}
