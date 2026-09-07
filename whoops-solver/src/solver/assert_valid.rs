use whoops_core::grid::{Grid, GridExt};
use whoops_core::offset::Offset;

use crate::Solver;

/// makes sure that the grid is valid
pub struct AssertValid;

impl<G> Solver<G> for AssertValid
where
    G: Grid,
{
    fn solve(&mut self, grid: G) -> Result<G, G> {
        // make sure no value tile sees more than expected
        let is_valid = grid
            .iter()
            .filter_map(|(pos, tile)| Some(pos).zip(tile.as_value()))
            .all(|(pos, value)| {
                let total: u8 = Offset::DIRECTIONS
                    .into_iter()
                    .map(|direction| {
                        grid.iter_tile_offset(pos, direction)
                            .skip(1)
                            .take_while(|tile| tile.is_dot())
                            .count() as u8
                    })
                    .sum();

                total <= value
            });

        if is_valid { Ok(grid) } else { Err(grid) }
    }
}
