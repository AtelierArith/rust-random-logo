//! Benchmarks for the rust-random-logo library

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use rust_random_logo::{rand_sigma_factor_ifs, render, Config};

fn bench_generate_ifs(c: &mut Criterion) {
    c.bench_function("generate_ifs", |b| {
        b.iter(|| {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            black_box(rand_sigma_factor_ifs(&mut rng))
        })
    });
}

fn bench_render_small(c: &mut Criterion) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    let ifs = rand_sigma_factor_ifs(&mut rng);

    let config = Config {
        height: 100,
        width: 100,
        npoints: 10_000,
        ifs_name: "SigmaFactorIFS".to_string(),
        ndims: 2,
        rng_name: "Xoshiro256PlusPlus".to_string(),
        seed: 42,
    };

    c.bench_function("render_small", |b| {
        b.iter(|| {
            let rng = Xoshiro256PlusPlus::seed_from_u64(42);
            black_box(render(rng, &ifs, &config))
        })
    });
}

fn bench_render_medium(c: &mut Criterion) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    let ifs = rand_sigma_factor_ifs(&mut rng);

    let config = Config {
        height: 384,
        width: 384,
        npoints: 100_000,
        ifs_name: "SigmaFactorIFS".to_string(),
        ndims: 2,
        rng_name: "Xoshiro256PlusPlus".to_string(),
        seed: 42,
    };

    c.bench_function("render_medium", |b| {
        b.iter(|| {
            let rng = Xoshiro256PlusPlus::seed_from_u64(42);
            black_box(render(rng, &ifs, &config))
        })
    });
}

criterion_group!(
    benches,
    bench_generate_ifs,
    bench_render_small,
    bench_render_medium
);
criterion_main!(benches);
