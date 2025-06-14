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
    pub duration: SumDuration,
    pub ptype: PartType,
    pub complexids: Vec<usize>,
}
