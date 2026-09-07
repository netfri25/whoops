use std::ops::{BitAnd, BitOr};

use whoops_core::pos::Pos;

pub mod block_complete;
pub mod func_tile_rule;
pub mod minimum;
pub mod violation;

mod ext;

pub use ext::*;
pub use func_tile_rule::*;

pub trait TileRule<G: ?Sized> {
    /// returns `None` when the rule wasn't able to be applied (usually because of out of bounds)
    fn solve_at(&mut self, pos: Pos, grid: &mut G) -> Option<Response>;
}

impl<R, G> TileRule<G> for Box<R>
where
    R: TileRule<G> + ?Sized,
    G: ?Sized,
{
    fn solve_at(&mut self, pos: Pos, grid: &mut G) -> Option<Response> {
        self.as_mut().solve_at(pos, grid)
    }
}

impl<R, G> TileRule<G> for &mut R
where
    R: TileRule<G> + ?Sized,
    G: ?Sized,
{
    fn solve_at(&mut self, pos: Pos, grid: &mut G) -> Option<Response> {
        (**self).solve_at(pos, grid)
    }
}

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
