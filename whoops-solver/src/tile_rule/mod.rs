use whoops_core::pos::Pos;

pub mod block_complete;

pub trait TileRule<G: ?Sized> {
    /// returns `None` when the rule wasn't able to be applied (usually because of out of bounds)
    fn solve_at(&self, pos: Pos, grid: &mut G) -> Option<Response>;
}

impl<R, G> TileRule<G> for Box<R>
where
    R: TileRule<G> + ?Sized,
    G: ?Sized,
{
    fn solve_at(&self, pos: Pos, grid: &mut G) -> Option<Response> {
        self.as_ref().solve_at(pos, grid)
    }
}

impl<R, G> TileRule<G> for &R
where
    R: TileRule<G> + ?Sized,
    G: ?Sized,
{
    fn solve_at(&self, pos: Pos, grid: &mut G) -> Option<Response> {
        (*self).solve_at(pos, grid)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Response {
    Keep,
    Consume,
}
