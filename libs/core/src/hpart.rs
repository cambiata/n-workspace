use crate::{
    barline::BarlineType,
    clef::ClefSignature,
    duration::SumDuration,
    key::KeySignature,
    time::TimeSignature,
    voice::{VecNoteIds, VecStemitemIds},
};

#[derive(Debug)]
pub struct HPartItemsRow {
    pub id: usize,
    pub hpart_ids: Vec<usize>,
    pub part_idx: usize,
}

#[derive(Debug)]
pub struct HPartItemsColumn {
    pub id: usize,
    pub hptype: HPartItemsColumnType,
    pub position: usize,
    pub duration: usize,
    pub col_idx: usize,
}

#[derive(Debug)]
pub enum HPartItemsColumnType {
    Clefs(Vec<usize>),
    Barlines(Vec<usize>),
    Musics(Vec<usize>),
}

#[derive(Debug, Clone)]
pub struct HPartItem {
    pub id: usize,
    pub parttype: HPartType,
    pub position: usize,
    pub duration: usize,
    pub part_idx: usize,
    pub col_idx: usize,
}

#[derive(Debug, Clone)]
pub enum HPartType {
    Clef(ClefSignature),
    Barline(BarlineType),
    Music(HPartMusicType, Vec<usize>, HPartAttributes),
}

#[derive(Debug, Clone)]
pub struct HPartAttributes {
    pub clef: ClefSignature,
    pub time: TimeSignature,
    pub key: KeySignature,
}

#[derive(Debug, Clone)]
pub enum HPartMusicType {
    TwoVoices { upper: VoiceType2, lower: VoiceType2 },
    OneVoice { voice: VoiceType2 },
}

#[derive(Debug, Clone)]
pub enum VoiceType2 {
    Barpause(SumDuration),
    NoteIds {
        note_ids: VecNoteIds,
        duration: SumDuration,
        stemitem_ids: VecStemitemIds,
    },
}
