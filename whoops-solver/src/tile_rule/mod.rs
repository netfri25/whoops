use whoops_core::pos::Pos;

pub mod block_complete;
pub mod func_tile_rule;
pub mod minimum;

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Response {
    Keep,
    Consume,
}
