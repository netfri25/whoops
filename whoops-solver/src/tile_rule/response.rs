use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, Copy, Default)]
pub struct Response {
    pub applied: bool,
    pub consumed: bool,
}

impl Response {
    pub fn apply(self, applied: bool) -> Self {
        Self { applied, ..self }
    }

    pub fn consume(self, consumed: bool) -> Self {
        Self { consumed, ..self }
    }
}

impl BitOr for Response {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            applied: self.applied | rhs.applied,
            consumed: self.consumed | rhs.consumed,
        }
    }
}

impl BitAnd for Response {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            applied: self.applied & rhs.applied,
            consumed: self.consumed & rhs.consumed,
        }
    }
}
