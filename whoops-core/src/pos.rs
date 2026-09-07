use crate::offset::Offset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn add_offset(self, offset: Offset) -> Option<Self> {
        Some(Self::new(
            self.x.checked_add_signed(offset.x)?,
            self.y.checked_add_signed(offset.y)?,
        ))
    }
}
