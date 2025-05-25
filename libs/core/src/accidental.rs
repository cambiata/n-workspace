#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Accidental {
    DoubleSharp,
    Sharp,
    None,
    Neutral,
    Flat,
    DoubleFlat,
}

impl Accidental {
    pub fn find(s: &str) -> Self {
        match s {
            s if s.contains("##") => Self::DoubleSharp,
            s if s.contains("bb") => Self::DoubleFlat,
            s if s.contains("#") => Self::Sharp,
            s if s.contains("n") => Self::Neutral,
            s if s.contains("b") => Self::Flat,
            _ => Self::None,
        }
    }

    pub fn parse(value: &str) -> Result<Accidental, Box<dyn std::error::Error>> {
        match value {
            "##" => Ok(Accidental::DoubleSharp),
            "bb" => Ok(Accidental::DoubleFlat),
            "#" => Ok(Accidental::Sharp),
            "n" => Ok(Accidental::Neutral),
            "b" => Ok(Accidental::Flat),
            "" => Ok(Accidental::None),
            _ => Err("Invalid accidental".into()),
        }
    }
}

impl Default for Accidental {
    fn default() -> Self {
        Accidental::None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let a: Accidental = Accidental::parse("b").unwrap();
        assert!(a == Accidental::Flat);
    }
}
