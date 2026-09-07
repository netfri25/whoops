use whoops_core::grid::Grid;

pub mod solver;
pub mod tile_rule;

pub use solver::*;

pub fn default_solver<G>() -> impl Solver<G>
where
    G: Grid,
{
    let rule = tile_rule::default_tile_rule();
    TileRuleSolver::new(rule)
        .and_then(Unreachable)
        .and_then(AssertFull)
        .and_then(AssertValid)
}
