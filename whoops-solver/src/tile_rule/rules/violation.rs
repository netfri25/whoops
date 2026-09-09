use whoops_core::grid::{Grid, GridExt};
use whoops_core::offset::Offset;
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;

use crate::tile_rule::{Response, TileRule};

pub struct Violation;

impl<G> TileRule<G> for Violation
where
    G: Grid,
{
    fn solve_at(&mut self, pos: Pos, grid: &mut G) -> Option<Response> {
        let value = grid.get(pos)?.as_value()?;

        let direction_visible_count = Offset::DIRECTIONS.map(|offset| {
            grid.iter_tile_offset(pos, offset)
                .skip(1)
                .take_while(|tile| tile.is_dot())
                .count() as u8
        });

        let total: u8 = direction_visible_count.iter().sum();

        // if dot sees the expected value (or more) then there's nothing to apply
        if total >= value {
            return Some(Response::default());
        }

        let leftover = value - total;

        let mut applied = false;
        for (direction, count) in Offset::DIRECTIONS.into_iter().zip(direction_visible_count) {
            let offset = direction * (count + 1) as i32;

            let Some(placement_pos) = pos.add_offset(offset) else {
                continue;
            };

            // placements only apply for "unknown" tiles
            if grid
                .get(placement_pos)
                .is_none_or(|tile| !tile.is_unknown())
            {
                continue;
            }

            let violates = grid
                .iter_tile_offset(placement_pos, direction)
                .skip(1)
                .take_while(|tile| tile.is_dot())
                .nth((leftover - 1) as usize)
                .is_some_and(|tile| tile.is_dot());

            if violates {
                applied = true;
                grid.set(placement_pos, Tile::Wall);
            }
        }

        Some(if applied { Response::Applied } else { Response::Ignored })
    }
}
