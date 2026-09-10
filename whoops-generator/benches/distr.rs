use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use whoops_core::grid::TileGrid;
use whoops_generator::random::{self, GridDistrbution};

fn benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(12);
    let w = 10;
    let h = 10;
    let params = random::Params::new(w, h);
    let distr = GridDistrbution {
        wall: params.wall_low_bound,
        width: params.width,
        height: params.height,
    };

    let name = format!("distr {w}x{h}");
    c.bench_function(&name, |b| {
        b.iter(|| rng.sample::<TileGrid, _>(black_box(distr)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
