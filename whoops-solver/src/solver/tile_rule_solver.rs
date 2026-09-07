use std::collections::VecDeque;

use whoops_core::grid::{Grid, GridExt};
use whoops_core::pos::Pos;

use crate::tile_rule::block_complete::BlockComplete;
use crate::{Solver};
use crate::tile_rule::{Response, TileRule};

pub struct TileRuleSolver<R> {
    tile_rule: R,
    targets: VecDeque<Pos>,
}

impl TileRuleSolver<DefaultTileRuleType> {
    pub fn with_default_rule() -> Self {
        Self::new(default_tile_rule())
    }
}

impl<R, G> Solver<G> for TileRuleSolver<R>
where
    R: TileRule<G>,
    G: Grid,
{
    fn solve(&mut self, mut grid: G) -> Result<G, G> {
        // positions to target (number tiles)
        let targets = grid
            .iter()
            .filter_map(|(pos, tile)| tile.as_value().map(|_| pos));

        self.targets.clear();
        self.targets.extend(targets);

        // using `unwrap_or(true)` makes it keep running even if tried to access out of bounds
        while self.step(&mut grid).unwrap_or(true) {}

        // if no more targets are left, it means that all of the targets were solved, and the entire
        // grid is now solved
        if self.targets.is_empty() {
            Ok(grid)
        } else {
            Err(grid)
        }
    }
}

impl<R> TileRuleSolver<R> {
    pub fn new(tile_rule: R) -> Self {
        let targets = VecDeque::new();
        Self {
            tile_rule,
            targets,
        }
    }

    /// returns `true` if another step should be applied.
    /// returns `false` when should stop stepping.
    /// returns `None` if tried to access out of bounds
    fn step<G>(&mut self, grid: &mut G) -> Option<bool>
    where
        R: TileRule<G>,
    {
        // if there are no more targets left, then stepping should stop
        let Some(start) = self.targets.pop_front() else {
            return Some(false);
        };

        let mut target = start;

        // try to apply rule to at least one target
        loop {
            if self.tile_rule.solve_at(target, grid)? == Response::Consume {
                return Some(true);
            }

            self.targets.push_back(target);
            target = self
                .targets
                .pop_front()
                .expect("front must exist after push");

            // if looped around without applying even once, then the grid can't be solved and
            // stepping should stop
            if target == start {
                return Some(false);
            }
        }
    }
}

pub type DefaultTileRuleType = BlockComplete;


pub fn default_tile_rule() -> DefaultTileRuleType {
    BlockComplete
    // Seq(&[
    //     &Unreachable as &dyn Rule<G>,
    //     &Minimum as &dyn Rule<G>,
    //     &Violation as &dyn Rule<G>,
    //     &BlockComplete as &dyn Rule<G>,
    // ])
}
