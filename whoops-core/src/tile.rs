use std::fmt;

/// the `Tile::Dot` variant with the value `0` acts as a user-placed value. this means that it
/// should always match with other dot values, no matter their value. this exact behavior is
/// implemented in the `Tile::matches` method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Unknown,
    Wall,
    Dot(u8),
}

impl Tile {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub fn is_wall(&self) -> bool {
        matches!(self, Self::Wall)
    }

    pub fn is_dot(&self) -> bool {
        matches!(self, Self::Dot(_))
    }

    pub fn as_value(&self) -> Option<u8> {
        if let &Self::Dot(value) = self {
            (value != 0).then_some(value)
        } else {
            None
        }
    }

    pub fn matches(&self, other: Self) -> bool {
        match (*self, other) {
            (Self::Dot(l), Self::Dot(r)) if l == 0 || r == 0 => true,
            (l, r) => l == r,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "."),
            Self::Wall => write!(f, "x"),
            Self::Dot(value) => write!(f, "{value}"),
        }
    }
}
