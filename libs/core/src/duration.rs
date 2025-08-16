use crate::{head::HeadType, rest::RestType};

pub type SumDuration = usize;

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum NoteDuration {
    D1Dot = 144,
    D1 = 96,
    D2Dot = 72,
    D2 = 48,
    D4Dot = 36,
    D2Tri = 32,
    D4 = 24,
    D8Dot = 18,
    D4Tri = 16,
    D8 = 12,
    D16Dot = 9,
    D8Tri = 8,
    D16 = 6,
    D16Tri = 4,
    D32 = 3,
}

impl TryFrom<usize> for NoteDuration {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            144 => Ok(NoteDuration::D1Dot),
            96 => Ok(NoteDuration::D1),
            72 => Ok(NoteDuration::D2Dot),
            48 => Ok(NoteDuration::D2),
            36 => Ok(NoteDuration::D4Dot),
            32 => Ok(NoteDuration::D2Tri),
            24 => Ok(NoteDuration::D4),
            18 => Ok(NoteDuration::D8Dot),
            16 => Ok(NoteDuration::D4Tri),
            12 => Ok(NoteDuration::D8),
            9 => Ok(NoteDuration::D16Dot),
            8 => Ok(NoteDuration::D8Tri),
            6 => Ok(NoteDuration::D16),
            4 => Ok(NoteDuration::D16Tri),
            3 => Ok(NoteDuration::D32),
            _ => Err(format!("Invalid duration value '{}'", value).into()),
        }
    }
}

impl NoteDuration {
    #[allow(dead_code)]
    pub fn is_beamable(self) -> bool {
        match self {
            NoteDuration::D8 | NoteDuration::D16 | NoteDuration::D32 => true,
            NoteDuration::D8Tri | NoteDuration::D16Tri => true,
            NoteDuration::D8Dot | NoteDuration::D16Dot => true,
            _ => false,
        }
    }

    pub fn is_dotted(self) -> bool {
        match self {
            NoteDuration::D1Dot | NoteDuration::D2Dot | NoteDuration::D4Dot | NoteDuration::D8Dot | NoteDuration::D16Dot => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn has_stem(self) -> bool {
        match self {
            NoteDuration::D1 | NoteDuration::D1Dot => false,
            _ => true,
        }
    }

    #[allow(dead_code)]
    pub fn has_flag(self) -> bool {
        match self {
            NoteDuration::D8 | NoteDuration::D8Dot | NoteDuration::D8Tri | NoteDuration::D16 | NoteDuration::D16Dot | NoteDuration::D16Tri | NoteDuration::D32 => true,
            _ => false,
        }
    }

    pub fn parse(s: &str) -> Result<NoteDuration, Box<dyn std::error::Error>> {
        let s = s.trim();
        if !(s.starts_with('D') || s.starts_with("d")) {
            return Err("Duration string must start with letter 'd'".into());
        }

        let value2 = &s[1..];
        match value2 {
            "1." => Ok(NoteDuration::D1Dot),
            "1" => Ok(NoteDuration::D1),

            "2." => Ok(NoteDuration::D2Dot),
            "2" => Ok(NoteDuration::D2),
            "2Tri" => Ok(NoteDuration::D2Tri),

            "4." => Ok(NoteDuration::D4Dot),
            "4" => Ok(NoteDuration::D4),

            "8." => Ok(NoteDuration::D8Dot),
            "4Tri" => Ok(NoteDuration::D4Tri),
            "8" => Ok(NoteDuration::D8),

            "16." => Ok(NoteDuration::D16Dot),
            "8Tri" => Ok(NoteDuration::D8Tri),
            "16" => Ok(NoteDuration::D16),
            "16Tri" => Ok(NoteDuration::D16Tri),

            "32" => Ok(NoteDuration::D32),
            _ => Err(format!("Invalid duration string '{}'", value2).into()),
        }
    }

    pub fn get_head_type(self) -> HeadType {
        match self {
            NoteDuration::D1 | NoteDuration::D1Dot => HeadType::Whole,
            NoteDuration::D2 | NoteDuration::D2Dot | NoteDuration::D2Tri => HeadType::White,
            _ => HeadType::Black,
        }
    }

    pub fn get_rest_type(self) -> RestType {
        match self {
            NoteDuration::D1 | NoteDuration::D1Dot => RestType::Whole,
            NoteDuration::D2 | NoteDuration::D2Dot | NoteDuration::D2Tri => RestType::Half,
            NoteDuration::D4 | NoteDuration::D4Dot | NoteDuration::D4Tri => RestType::Quarter,
            NoteDuration::D8 | NoteDuration::D8Dot | NoteDuration::D8Tri => RestType::Eighth,
            NoteDuration::D16 | NoteDuration::D16Dot | NoteDuration::D16Tri => RestType::Sixteenth,
            _ => todo!(),
        }
    }

    pub fn get_base_value(self) -> u8 {
        match self {
            NoteDuration::D1Dot | NoteDuration::D1 => 1,
            NoteDuration::D2Dot | NoteDuration::D2 | NoteDuration::D2Tri => 2,
            NoteDuration::D4Dot | NoteDuration::D4Tri | NoteDuration::D4 => 4,
            NoteDuration::D8Dot | NoteDuration::D8Tri | NoteDuration::D8 => 8,
            NoteDuration::D16Dot | NoteDuration::D16Tri | NoteDuration::D16 => 16,
            NoteDuration::D32 => 32,
        }
    }
}

impl Default for NoteDuration {
    fn default() -> Self {
        NoteDuration::D4
    }
}

pub struct DurationUtils;
impl DurationUtils {
    pub fn durations_smallest_base_value(durations: &[NoteDuration]) -> i8 {
        let mut smallest = 0;
        for duration in durations {
            smallest = smallest.max(duration.get_base_value() as i8);
        }
        smallest
    }

    pub fn get_base_values(durations: &[NoteDuration]) -> Vec<i8> {
        durations.iter().map(|d| d.get_base_value() as i8).collect()
    }
}

// pub fn duration_from(v: usize) -> Result<Duration> {
//     match v {
//         NV1DOT | NV1 | NV2DOT | NV2 | NV4DOT | NV2TRI | NV4 | NV8DOT | NV4TRI | NV8 | NV16DOT | NV8TRI | NV16 | NV16TRI | NV32 => Ok(v),
//         _ => Err(Generic(format!("Can not convert value {} to usize Duration", v)).into()),
//     }
// }

// pub fn get_headtype_from_duration(duration: Duration) -> HeadType {
//     match duration {
//         NV1 | NV1DOT => HeadType::Unfilled,
//         NV2 | NV2DOT | NV2TRI => HeadType::Unfilled,
//         _ => HeadType::Filled,
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() -> Result<(), Box<dyn std::error::Error>> {
        print!("Hello, world!");
        let d: NoteDuration = NoteDuration::parse("D16Tri")?;
        assert!(d == NoteDuration::D16Tri);
        Ok(())
    }
}
