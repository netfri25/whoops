use whoops_core::grid::{Grid, GridIter};
use whoops_core::offset::Offset;
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;

use crate::tile_rule::{Response, TileRule};

pub struct Minimum;

impl<G> TileRule<G> for Minimum
where
    G: GridIter,
{
    fn solve_at(&mut self, pos: Pos, grid: &mut G) -> Option<Response> {
        let value = grid.get(pos)?.as_value()?;

        let value = value as usize;

        let max_in_direction = Offset::DIRECTIONS.map(|offset| {
            grid.iter_tile_offset(pos, offset)
                .skip(1)
                .take_while(|tile| !tile.is_wall())
                .count()
        });

        let total: usize = max_in_direction.iter().sum();

        let mut applied = false;
        for (direction, count) in Offset::DIRECTIONS.into_iter().zip(max_in_direction) {
            let rest_sum = total - count;
            let leftover = value.saturating_sub(rest_sum);

            for i in 1..=leftover {
                let dot_offset = direction * i as i32;
                let Some(dot_position) = pos.add_offset(dot_offset) else {
                    // since pos is a valid position in the grid, if adding to it some offset fails,
                    // and that offset magnitude only gets larger every iteration, then there's no
                    // reason to keep on iterating since the rest will also fail.
                    break;
                };

                let Some(target_tile) = grid.get(dot_position) else {
                    break;
                };

                // allow placing only on unknown tiles.
                // this makes dot tiles with a value not lose their value.
                if !target_tile.is_unknown() {
                    continue;
                }

                applied = true;
                grid.set(dot_position, Tile::Dot(0));
            }
        }

        Some(if applied { Response::Applied } else { Response::Ignored })
    }
}
