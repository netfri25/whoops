use whoops_core::grid::{Grid, GridExt};

use crate::{Solver, UpdateValue};

/// update the value of each of the dot tiles in the grid
pub struct UpdateAllValues;

impl<G> Solver<G> for UpdateAllValues
where
    G: Grid,
{
    #[inline(always)]
    fn solve(&mut self, mut grid: G) -> Result<G, G> {
        for pos in grid.iter_pos() {
            grid = UpdateValue(pos).solve(grid).unwrap_or_else(|grid| grid);
        }

        Ok(grid)
    }
}
