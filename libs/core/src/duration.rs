pub type SumDuration = usize;

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Duration {
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

impl Duration {
    #[allow(dead_code)]
    pub fn is_beamable(self) -> bool {
        match self {
            Duration::D8 | Duration::D16 | Duration::D32 => true,
            Duration::D8Tri | Duration::D16Tri => true,
            Duration::D8Dot | Duration::D16Dot => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn has_stem(self) -> bool {
        match self {
            Duration::D1 | Duration::D1Dot => false,
            _ => true,
        }
    }

    pub fn parse(s: &str) -> Result<Duration, Box<dyn std::error::Error>> {
        let s = s.trim();
        if !(s.starts_with('D') || s.starts_with("d")) {
            return Err("MustStartWithD".into());
        }

        let value2 = &s[1..];
        match value2 {
            "1." => Ok(Duration::D1Dot),
            "1" => Ok(Duration::D1),

            "2." => Ok(Duration::D2Dot),
            "2" => Ok(Duration::D2),
            "2Tri" => Ok(Duration::D2Tri),

            "4." => Ok(Duration::D4Dot),
            "4" => Ok(Duration::D4),

            "8." => Ok(Duration::D8Dot),
            "4Tri" => Ok(Duration::D4Tri),
            "8" => Ok(Duration::D8),

            "16." => Ok(Duration::D16Dot),
            "8Tri" => Ok(Duration::D8Tri),
            "16" => Ok(Duration::D16),
            "16Tri" => Ok(Duration::D16Tri),

            "32" => Ok(Duration::D32),
            _ => Err("Invalid duration".into()),
        }
    }
}

impl Default for Duration {
    fn default() -> Self {
        Duration::D4
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
    fn test() {
        print!("Hello, world!");
        let d: Duration = Duration::parse("D16Tri").unwrap();
        assert!(d == Duration::D16Tri);
    }
}
