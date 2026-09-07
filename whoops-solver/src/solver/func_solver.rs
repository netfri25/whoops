use super::Solver;

/// an adapter for functions (FnMut) to implement the `Solver` trait
pub struct FuncSolver<F>(pub F);

impl<F, G> Solver<G> for FuncSolver<F>
where
    F: FnMut(G) -> Result<G, G>,
{
    fn solve(&mut self, grid: G) -> Result<G, G> {
        (self.0)(grid)
    }
}
