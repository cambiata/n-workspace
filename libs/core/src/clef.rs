#[derive(Debug, Clone)]

pub enum ClefSignature {
    None,
    Treble,
    Bass,
    Alto,
    Tenor,
    Percussion,
}

impl ClefSignature {
    pub fn find(s: &str) -> Self {
        if s.contains("G") {
            ClefSignature::Treble
        } else if s.contains("F") {
            ClefSignature::Bass
        } else if s.contains("C") {
            ClefSignature::Alto
        } else if s.contains("T") {
            ClefSignature::Tenor
        } else if s.contains("P") {
            ClefSignature::Percussion
        } else {
            ClefSignature::None
        }
    }
}
