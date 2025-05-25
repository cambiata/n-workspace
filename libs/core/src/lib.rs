pub mod accidental;
pub mod clef;
pub mod context;
pub mod direction;
pub mod duration;
pub mod head;
pub mod key;
pub mod note;
pub mod part;
pub mod sysitem;
pub mod voice;

pub type ItemId = usize;

pub type ScoreError = Box<dyn std::error::Error>;
