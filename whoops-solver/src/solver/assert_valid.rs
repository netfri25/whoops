use whoops_core::grid::GridIter;
use whoops_core::offset::Offset;
use whoops_core::tile::Tile;

use crate::Solver;

/// makes sure that the grid is valid
pub struct AssertValid;

impl<G> Solver<G> for AssertValid
where
    G: GridIter,
{
    fn solve(&mut self, grid: G) -> Result<G, G> {
        // make sure no value tile sees more than expected
        let is_valid = grid.iter().all(|(pos, tile)| {
            match tile {
                // wall tiles and unknown tiles are always valid
                Tile::Wall | Tile::Unknown => true,

                // if there's no value (a user-placed dot), make sure that it may see at least one
                // other dot - at least one immediate neighbor is not a wall
                Tile::Dot(0) => Offset::DIRECTIONS
                    .into_iter()
                    .filter_map(|off| pos.add_offset(off))
                    .filter_map(|pos| grid.get(pos))
                    .any(|tile| !tile.is_wall()),

                // if there's a value, make sure that the dot doesn't see more than its value
                Tile::Dot(value) => {
                    let total_visible: u8 = Offset::DIRECTIONS
                        .into_iter()
                        .map(|direction| {
                            grid.iter_tile_offset(pos, direction)
                                .skip(1)
                                .take_while(|tile| tile.is_dot())
                                .count() as u8
                        })
                        .sum();

                    value >= total_visible
                }
            }
        });

        if is_valid { Ok(grid) } else { Err(grid) }
    }
}
