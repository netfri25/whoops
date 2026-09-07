use whoops_core::grid::{Grid, GridExt};

use crate::Solver;

/// makes sure that the grid is full
pub struct AssertFull;

impl<G> Solver<G> for AssertFull
where
    G: Grid,
{
    fn solve(&mut self, grid: G) -> Result<G, G> {
        let contains_unknown = grid.iter_tile().any(|tile| tile.is_unknown());
        if contains_unknown {
            Err(grid)
        } else {
            Ok(grid)
        }
    }
}
