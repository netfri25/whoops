use criterion::*;
use rand::prelude::*;
use whoops_core::grid::TileGrid;
use whoops_generator::Generator;
use whoops_generator::random::{self, RandomGenerator};

fn random_generator_benchmark(c: &mut Criterion) {
    let rng = SmallRng::seed_from_u64(12);
    let w = 10;
    let h = 10;
    let params = random::Params::new(w, h);

    let name = format!("random generator {w}x{h}");
    let mut generator = RandomGenerator::new(rng, params);
    c.bench_function(&name, |b| {
        b.iter(|| Generator::<TileGrid>::generate(&mut generator))
    });
}

criterion_group!(benches, random_generator_benchmark);
criterion_main!(benches);
