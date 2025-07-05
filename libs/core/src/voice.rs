use crate::duration::SumDuration;

pub type VecNoteIds = Vec<usize>;
pub type VecStemitemIds = Vec<usize>;

#[derive(Debug)]
pub enum VoiceType {
    Barpause,
    NoteIds(VecNoteIds, SumDuration, VecStemitemIds),
}

#[derive(Debug)]
pub struct VoiceItem {
    pub duration: SumDuration,
    pub vtype: VoiceType,
}
