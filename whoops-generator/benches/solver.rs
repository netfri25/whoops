use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use std::hint::black_box;
use whoops_core::grid::TileGrid;
use whoops_generator::Generator;
use whoops_generator::random::{self, RandomGenerator};
use whoops_solver::{AssertFull, Solver, SolverExt, default_solver};

fn default_solver_benchmark(c: &mut Criterion) {
    let rng = SmallRng::seed_from_u64(12);
    let w = 10;
    let h = w;
    let grids_count = 2000;
    let params = random::Params::new(w, h);

    let mut generator = RandomGenerator::new(rng, params);
    let grids: Vec<TileGrid> = (0..grids_count)
        .map(|_| (&mut generator).generate().grid)
        .collect();

    let mut solver = default_solver().and_then(AssertFull);
    let name = format!("solver {grids_count} of {w}x{h}");
    c.bench_function(&name, |b| {
        b.iter(|| {
            for grid in &grids {
                solver.solve(black_box(grid.clone())).ok();
            }
        })
    });
}

criterion_group!(benches, default_solver_benchmark);
criterion_main!(benches);
