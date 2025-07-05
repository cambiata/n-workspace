use crate::{
    barline::BarlineType,
    clef::ClefSignature,
    duration::SumDuration,
    key::KeySignature,
    time::TimeSignature,
    voice::{VecNoteIds, VecStemitemIds},
};

#[derive(Debug)]
pub struct HPartItems {
    pub id: usize,
    pub hpart_ids: Vec<usize>,
}

#[derive(Debug)]
pub struct HPartItem {
    pub id: usize,
    pub parttype: HPartType,
    pub position: usize,
    pub duration: usize,
    pub part_idx: usize,
}

#[derive(Debug)]
pub enum HPartType {
    Clef(ClefSignature),
    Barline(BarlineType),
    Music(HPartMusicType, HPartAttributes),
}

#[derive(Debug)]
pub struct HPartAttributes {
    pub clef: ClefSignature,
    pub time: TimeSignature,
    pub key: KeySignature,
}

#[derive(Debug)]
pub enum HPartMusicType {
    TwoVoices { upper: VoiceType2, lower: VoiceType2, complexes: Vec<usize> },
    OneVoice { voice: VoiceType2, complexes: Vec<usize> },
}

#[derive(Debug)]
pub enum VoiceType2 {
    Barpause(SumDuration),
    NoteIds {
        note_ids: VecNoteIds,
        duration: SumDuration,
        stemitem_ids: VecStemitemIds,
    },
}
