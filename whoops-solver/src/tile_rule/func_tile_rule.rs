use whoops_core::pos::Pos;

use crate::tile_rule::{Response, TileRule};

pub struct FuncTileRule<F>(pub F);

impl<F, G> TileRule<G> for FuncTileRule<F>
where
    F: for<'a> FnMut(Pos, &'a mut G) -> Option<Response>,
{
    #[inline(always)]
    fn solve_at(&mut self, pos: Pos, grid: &mut G) -> Option<Response> {
        (self.0)(pos, grid)
    }
}
