#[derive(Debug)]

pub enum ClefSignature {
    None,
    Treble,
    Bass,
    Alto,
}

impl ClefSignature {
    pub fn find(s: &str) -> Self {
        if s.contains("G") {
            ClefSignature::Treble
        } else if s.contains("F") {
            ClefSignature::Bass
        } else if s.contains("C") {
            ClefSignature::Alto
        } else {
            ClefSignature::None
        }
    }
}
