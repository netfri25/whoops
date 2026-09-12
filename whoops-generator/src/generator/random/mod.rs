use rand::prelude::*;
use whoops_core::grid::{GridExt, GridIter, TileGrid};
use whoops_core::tile::Tile;
use whoops_solver::Solver;

use crate::generator::{Generator, GeneratorOutput};

mod distr;
mod params;

pub use distr::GridDistrbution;
pub use params::Params;

pub struct RandomGenerator<R, S> {
    rng: R,
    solver: S,
    params: Params,
}

impl<R, S> RandomGenerator<R, S> {
    pub fn new(rng: R, solver: S, params: Params) -> Self {
        Self { rng, solver, params }
    }

    // returns (knowns_count, Grid) where the Grid is the actual output, and the `knowns_count` is
    // returned just for efficiency, so that the caller won't have to count the knowns again.
    fn find_minimal_knowns<G>(&mut self, solution: &G, max_known: usize) -> (usize, G)
    where
        R: Rng,
        S: Solver<G>,
        G: GridIter + Clone,
    {
        // assumes that the solution grid contains only known tiles, which means that every position
        // contains a known tile
        let mut knowns: Vec<_> = solution.iter_pos().collect();

        // the output grid that will end up with some of its tiles unknown
        let mut grid = solution.clone();
        let mut knowns_count = knowns.len();

        // tries to remove every tile on the grid in random order (shuffled).
        // if removing the tile makes the grid unsolveble, restore that tile.
        // keeps trying to remove the tiles until `knowns_count <= max_known`, or until there are no
        // more tiles left to remove.
        knowns.shuffle(&mut self.rng);

        for pos in knowns {
            if knowns_count <= max_known {
                break;
            }

            let Some(old_tile) = grid.set(pos, Tile::Unknown) else {
                continue;
            };

            let solvable = self.solver
                .solve(grid.clone())
                .is_ok_and(|solved| solved.matches(solution));

            if solvable {
                // accept deletion
                knowns_count -= 1;
            } else {
                // revert deletion
                grid.set(pos, old_tile);
            }
        }

        (knowns_count, grid)
    }
}

impl<R, S, G> Generator<G> for RandomGenerator<R, S>
where
    R: Rng,
    S: Solver<G>,
    G: From<TileGrid> + GridIter + Clone,
{
    fn generate(mut self) -> GeneratorOutput<G> {
        (&mut self).generate()
    }
}

impl<R, S, G> Generator<G> for &mut RandomGenerator<R, S>
where
    R: Rng,
    S: Solver<G>,
    G: From<TileGrid> + GridIter + Clone,
{
    fn generate(self) -> GeneratorOutput<G> {
        let distr = GridDistrbution {
            width: self.params.width,
            height: self.params.height,
            wall: self.params.wall_low_bound,
        };

        let area = self.params.width * self.params.height;
        let max_known = (self.params.known_high_bound * area as f32).floor() as usize;

        let mut minimal_output = None;
        let mut minimal_knowns_count = usize::MAX;
        for _ in 0..self.params.iterations {
            let solution = self.rng.sample::<TileGrid, _>(distr).into();
            let (knowns_count, grid) = self.find_minimal_knowns(&solution, max_known);
            let output = GeneratorOutput { grid, solution };

            if knowns_count <= max_known {
                return output;
            }

            if knowns_count < minimal_knowns_count {
                minimal_output = Some(output);
                minimal_knowns_count = knowns_count;
            }
        }

        minimal_output.expect("more than one iteration ensures that there's an output")
    }
}
