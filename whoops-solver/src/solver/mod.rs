mod tile_rule_solver;

pub use tile_rule_solver::{DefaultTileRuleType, TileRuleSolver, default_tile_rule};

mod ext;
mod func_solver;

pub use ext::*;
pub use func_solver::*;

/// some type that is able to solve grids
///
/// for a "step by step" solver, you can give it a `History<G>`, and later convert it to `Lazy<G>`
/// with the history as the steps. the `impl From<History<G>> for Lazy<G>` has that behavior.
pub trait Solver<G> {
    /// returns Ok(G) if nothing went wrong, or
    /// returns Err(G) with a partially solved grid, when it didn't succeed
    fn solve(&mut self, grid: G) -> Result<G, G>;
}

impl<S, G> Solver<G> for &mut S
where
    S: Solver<G>,
{
    fn solve(&mut self, grid: G) -> Result<G, G> {
        (**self).solve(grid)
    }
}

impl<S, G> Solver<G> for Box<S>
where
    S: Solver<G>,
{
    fn solve(&mut self, grid: G) -> Result<G, G> {
        (**self).solve(grid)
    }
}
