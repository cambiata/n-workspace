use crate::{duration::SumDuration, voice::VoiceItem};

pub type PartId = usize;

#[derive(Debug)]
pub enum PartType {
    // Barpause,
    OneVoice(VoiceItem),
    TwoVoice(VoiceItem, VoiceItem),
    OtherPart,
}

#[derive(Debug)]
pub struct PartItem {
    pub id: PartId,
    pub idx: usize,
    pub duration: SumDuration,
    pub ptype: PartType,
    pub complexids: Vec<usize>,
}
