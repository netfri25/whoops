mod assert_full;
mod assert_valid;
mod assert_matches;
mod ext;
mod func_solver;
mod tile_rule_solver;
mod unreachable;
mod update_all_values;
mod update_value;

pub use assert_full::AssertFull;
pub use assert_valid::AssertValid;
pub use assert_matches::AssertMatches;
pub use ext::*;
pub use func_solver::*;
pub use tile_rule_solver::TileRuleSolver;
pub use unreachable::Unreachable;
pub use update_all_values::UpdateAllValues;
pub use update_value::UpdateValue;

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
    S: Solver<G> + ?Sized,
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

impl<S, G> Solver<G> for Option<S>
where
    S: Solver<G>
{
    fn solve(&mut self, grid: G) -> Result<G, G> {
        let Some(solver) = self.as_mut() else {
            return Ok(grid);
        };

        solver.solve(grid)
    }
}
