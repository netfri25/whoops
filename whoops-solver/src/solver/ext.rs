use crate::{FuncSolver, Solver};

pub trait SolverExt<G>: Solver<G> {
    fn and_then<B>(self, other: B) -> impl Solver<G>
    where
        B: Solver<G>;

    fn or_else<B>(self, other: B) -> impl Solver<G>
    where
        B: Solver<G>;
}

impl<S, G> SolverExt<G> for S
where
    S: Solver<G>,
{
    fn and_then<B>(mut self, mut other: B) -> impl Solver<G>
    where
        B: Solver<G>,
    {
        FuncSolver(move |grid| self.solve(grid).and_then(|grid| other.solve(grid)))
    }

    fn or_else<B>(mut self, mut other: B) -> impl Solver<G>
    where
        B: Solver<G>,
    {
        FuncSolver(move |grid| self.solve(grid).or_else(|grid| other.solve(grid)))
    }
}
