use rand::distr::Bernoulli;
use rand::prelude::*;
use whoops_core::grid::{Grid, GridIter, TileGrid};
use whoops_core::offset::Offset;
use whoops_core::tile::Tile;
use whoops_solver::{Solver, UpdateAllValues, UpdateValue};

/// disribution for generating random valid grids with values and walls only
#[derive(Clone, Copy)]
pub struct GridDistrbution {
    pub width: u32,
    pub height: u32,
    pub wall: f32,
}

impl Distribution<TileGrid> for GridDistrbution {
    #[inline(always)]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileGrid {
        // start with a grid that has randomally generated walls
        let wall_prob = self.wall.clamp(0., 1.);
        let wall_dist = Bernoulli::new(wall_prob as f64).expect("always [0, 1]");
        let tiles: Box<[Tile]> = rng
            .sample_iter(wall_dist)
            .map(|is_wall| if is_wall { Tile::Wall } else { Tile::Dot(0) })
            .take((self.width * self.height) as usize)
            .collect();

        let grid = TileGrid::from_parts(tiles, self.width).expect("width divides tiles.len()");

        // since we allow non-square grids, it makes the most sense to calculate
        // the "size" as a geometric mean.
        // https://en.wikipedia.org/wiki/Geometric_mean
        let size = (self.width * self.height).isqrt();

        // add counts for every dot on the grid
        let mut grid = UpdateAllValues.solve(grid).unwrap_or_else(|grid| grid);

        loop {
            // find the positions of all of the values that are bigger than `size`
            let highest_value_pos = grid
                .iter()
                .filter_map(|(pos, tile)| Some(pos).zip(tile.as_value()))
                .max_by_key(|(_, value)| *value)
                .and_then(|(pos, value)| (value as u32 > size).then_some(pos));

            // replace tile with wall
            let Some(pos) = highest_value_pos else {
                break;
            };

            grid.set(pos, Tile::Wall);

            for direction in Offset::DIRECTIONS {
                for pos in grid.iter_pos_offset(pos, direction).skip(1) {
                    if grid.get(pos).is_some_and(|tile| tile.is_wall()) {
                        break;
                    }

                    UpdateValue(pos).solve(&mut grid).ok();
                }
            }
        }

        grid
    }
}
