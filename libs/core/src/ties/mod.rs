#[derive(Debug)]
pub enum TieTo {
    Level(i8),
}

impl TieTo {
    pub fn find(s: &str, level: i8) -> Option<Self> {
        if s.starts_with("_") {
            return Some(TieTo::Level(level));
        }
        None
    }
}

#[derive(Debug)]
pub enum TieFrom {
    Level(i8),
}

impl TieFrom {
    pub fn find(s: &str, level: i8) -> Option<Self> {
        if s.ends_with("_") {
            return Some(TieFrom::Level(level));
        }
        None
    }
}

#[derive(Debug)]
pub enum CheckedTieFrom {
    Resolved(i8),
    Unresolved(i8),
}

#[derive(Debug)]
pub enum CheckedTieTo {
    Resolved(i8),
    Unresolved(i8),
}
