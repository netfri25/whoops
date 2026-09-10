use std::fmt;

/// the `Tile::Dot` variant with the value `0` acts as a user-placed value. this means that it
/// should always match with other dot values, no matter their value. this exact behavior is
/// implemented in the `Tile::matches` method.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Unknown,
    Wall,
    Dot(u8),
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Unknown => write!(f, "_"),
            Self::Wall => write!(f, "x"),
            Self::Dot(value) => {
                if value == 0 {
                    write!(f, "o")
                } else {
                    write!(f, "{value}")
                }
            }
        }
    }
}

impl Tile {
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    #[inline(always)]
    pub fn is_wall(&self) -> bool {
        matches!(self, Self::Wall)
    }

    #[inline(always)]
    pub fn is_dot(&self) -> bool {
        matches!(self, Self::Dot(_))
    }

    #[inline(always)]
    pub fn as_value(&self) -> Option<u8> {
        if let &Self::Dot(value) = self {
            (value != 0).then_some(value)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn is_value(&self) -> bool {
        self.as_value().is_some()
    }

    #[inline(always)]
    pub fn matches(&self, other: Self) -> bool {
        *self == other || matches!(self, Self::Dot(0)) && other.is_value()
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
