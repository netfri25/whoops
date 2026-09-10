use whoops_core::grid::GridIter;

use crate::Solver;

pub struct AssertMatches<P>(pub P);

impl<Pattern, G> Solver<G> for AssertMatches<Pattern>
where
    G: GridIter,
    Pattern: GridIter,
{
    fn solve(&mut self, grid: G) -> Result<G, G> {
        if grid.width() != self.0.width() || grid.height() != self.0.height() {
            return Err(grid);
        }

        let matches = grid
            .iter_tile()
            .zip(self.0.iter_tile())
            .all(|(target, solution)| target.matches(solution));

        if matches { Ok(grid) } else { Err(grid) }
    }
}
