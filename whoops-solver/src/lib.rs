use whoops_core::grid::GridIter;

pub mod solver;
pub mod tile_rule;

pub use solver::*;

pub fn default_solver<G>() -> impl Solver<G>
where
    G: GridIter,
{
    let rule = tile_rule::default_tile_rule();
    TileRuleSolver::new(rule).and_then(Unreachable)
}

pub fn default_solver_checked<G>() -> impl Solver<G>
where
    G: GridIter,
{
    default_solver().and_then(AssertFull).and_then(AssertValid)
}
