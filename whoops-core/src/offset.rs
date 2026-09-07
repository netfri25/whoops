use std::ops::{Add, Sub, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset {
    pub x: i32,
    pub y: i32
}

impl Offset {
    pub const DIRECTIONS: [Offset; 4] = [
        Self::new(-1, 0),
        Self::new(0, -1),
        Self::new(1, 0),
        Self::new(0, 1),
    ];

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Neg for Offset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Offset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Offset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Offset {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
