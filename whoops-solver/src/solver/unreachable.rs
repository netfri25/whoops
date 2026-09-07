use whoops_core::grid::{Grid, GridExt};
use whoops_core::offset::Offset;
use whoops_core::tile::Tile;

use crate::Solver;

/// Solver that marks all unreachable tiles as `Tile::Wall`
pub struct Unreachable;

impl<G> Solver<G> for Unreachable
where
    G: Grid,
{
    fn solve(&mut self, mut grid: G) -> Result<G, G> {
        for pos in grid.iter_pos() {
            if grid.get(pos).is_none_or(|tile| !tile.is_unknown()) {
                continue;
            }

            let blocked = Offset::DIRECTIONS.into_iter().all(|direction| {
                grid.iter_tile_offset(pos, direction)
                    .skip(1)
                    .find(|tile| !tile.is_unknown())
                    .is_none_or(|tile| tile.is_wall())
            });

            if blocked {
                grid.set(pos, Tile::Wall);
            }
        }

        Ok(grid)
    }
}
