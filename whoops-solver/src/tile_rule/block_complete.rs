use whoops_core::pos::Pos;

use crate::tile_rule::{TileRule, Response};


pub struct BlockComplete;

impl<G> TileRule<G> for BlockComplete {
    fn solve_at(&mut self, pos: Pos, grid: &mut G) -> Option<Response> {
        todo!()
    }
}
