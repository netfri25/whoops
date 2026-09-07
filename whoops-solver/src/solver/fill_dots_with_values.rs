use whoops_core::grid::{Grid, GridExt};
use whoops_core::offset::Offset;
use whoops_core::tile::Tile;

use crate::Solver;

pub struct FillDotsWithValues;

impl<G> Solver<G> for FillDotsWithValues
where
    G: Grid,
{
    fn solve(&mut self, mut grid: G) -> Result<G, G> {
        for pos in grid.iter_pos() {
            if !matches!(grid.get(pos), Some(Tile::Dot(0))) {
                continue;
            }

            let total = Offset::DIRECTIONS
                .into_iter()
                .map(|direction| {
                    grid.iter_tile_offset(pos, direction)
                        .skip(1)
                        .take_while(|tile| tile.is_dot())
                        .count() as u8
                })
                .sum();

            grid.set(pos, Tile::Dot(total));
        }

        Ok(grid)
    }
}
