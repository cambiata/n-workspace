use crate::{duration::SumDuration, voice::VoiceItem, ItemId};

#[derive(Debug)]
pub enum PartType {
    OneVoice(VoiceItem),
    TwoVoice(VoiceItem, VoiceItem),
    OtherPart,
}

#[derive(Debug)]
pub struct PartItem {
    pub id: ItemId,
    pub idx: usize,
    pub duration: SumDuration,
    pub position: usize,
    pub ptype: PartType,
    pub complexids: Vec<usize>,
}
