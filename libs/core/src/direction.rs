#[derive(Debug, Clone, PartialEq)]
pub enum DirectionUD {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DirectionUAD {
    Auto,
    Up,
    Down,
}

impl DirectionUAD {
    pub fn from_level(l: i8) -> DirectionUD {
        match l {
            l if l > 0 => DirectionUD::Up,
            _ => DirectionUD::Down,
        }
    }
}
