use std::collections::VecDeque;

use whoops_core::grid::GridIter;
use whoops_core::pos::Pos;

use crate::Solver;
use crate::tile_rule::TileRule;

pub struct TileRuleSolver<R> {
    tile_rule: R,
    targets: VecDeque<Pos>,
}

impl<R, G> Solver<G> for TileRuleSolver<R>
where
    R: TileRule<G>,
    G: GridIter,
{
    fn solve(&mut self, mut grid: G) -> Result<G, G> {
        // positions to target (number tiles)
        let targets = grid
            .iter()
            .filter_map(|(pos, tile)| tile.is_value().then_some(pos));

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
        Self { tile_rule, targets }
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
            let response = self.tile_rule.solve_at(target, grid)?;

            if !response.is_consumed() {
                self.targets.push_back(target);
            }

            if response.is_applied() {
                return Some(true);
            }

            let Some(new_target) = self.targets.pop_front() else {
                // no more targets left
                return Some(false);
            };

            target = new_target;

            // if looped around without applying even once, then the grid can't be solved and
            // stepping should stop
            if target == start {
                return Some(false);
            }
        }
    }
}
