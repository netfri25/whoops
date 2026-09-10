use whoops_core::grid::GridIter;
use whoops_core::offset::Offset;
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;

use crate::Solver;

/// update the value of a dot in a given position
pub struct UpdateValue(pub Pos);

impl<G> Solver<G> for UpdateValue
where
    G: GridIter,
{
    fn solve(&mut self, mut grid: G) -> Result<G, G> {
        let pos = self.0;

        // when not given a dot, return Err(grid)
        if !grid.get(pos).is_some_and(|tile| tile.is_dot()) {
            return Err(grid);
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

        let tile = if total == 0 {
            Tile::Wall
        } else {
            Tile::Dot(total)
        };

        grid.set(pos, tile);

        Ok(grid)
    }
}
