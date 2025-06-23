use crate::duration::SumDuration;

type VecNoteIds = Vec<usize>;
type VecStemitemIds = Vec<usize>;

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
