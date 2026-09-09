use whoops_core::grid::{Grid, GridExt};
use whoops_core::offset::Offset;
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;

use crate::tile_rule::{Response, TileRule};

pub struct BlockComplete;

impl<G> TileRule<G> for BlockComplete
where
    G: Grid,
{
    fn solve_at(&mut self, pos: Pos, grid: &mut G) -> Option<Response> {
        let n = grid.get(pos)?.as_value()?;

        let direction_visible_count = Offset::DIRECTIONS.map(|offset| {
            grid.iter_tile_offset(pos, offset)
                .skip(1) // skip the tile at "pos", since the count doesn't include the start tile
                .take_while(|tile| tile.is_dot())
                .count() as u8
        });

        let total: u8 = direction_visible_count.iter().sum();

        // if the tile doesn't see the exact required amount, it hasn't been completed yet, so
        // there's nothing to block and the tile should be kept for later iterations.
        if total != n {
            return Some(Response::default());
        }

        // here the tile is completed, so we place the red tiles around it to block its view
        for (direction, count) in Offset::DIRECTIONS.into_iter().zip(direction_visible_count) {
            // the wall should be placed after the dots. if there are 3 dots, the wall should be
            // placed in the 4th position.
            let offset = direction * (count as i32 + 1);
            let Some(wall_position) = pos.add_offset(offset) else {
                // if the wall position is less than 0 in either of its axis, we should just ignore
                // it and continue to the next iteration.
                continue;
            };

            if let Some(target_tile) = grid.get(wall_position)
                && target_tile.is_unknown()
            {
                grid.set(wall_position, Tile::Wall);
            }
        }

        // after placing the walls around the completed tile, there's no use to checking that tile
        // ever again, and we can mark it as completed.
        Some(Response::Consumed)
    }
}
