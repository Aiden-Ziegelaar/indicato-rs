use criterion::{criterion_group, criterion_main, Criterion};

use indicato_rs::signals::WildersSmoothing;
use indicato_rs::traits::Apply;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Apply 0..10k to Wilders 14 period", |b| {
        b.iter(|| {
            let mut ws = WildersSmoothing::new(14).unwrap();
            for x in 0..10_000 {
                ws.apply(x as f64);
            }
        })
    });

    c.bench_function("Apply 0..10k to Wilders 28 period", |b| {
        b.iter(|| {
            let mut ws = WildersSmoothing::new(28).unwrap();
            for x in 0..10_000 {
                ws.apply(x as f64);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);