use benches::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    for size in [0, 2, 10, 64, 1024, 20_000, 300_000, 1_000_000] {
        let (sample, _) = generate_sample(size);

        c.bench_function(&format!("xor {}", size), |b| b.iter(|| find_xor(&sample)));
        c.bench_function(&format!("HashMap {}", size), |b| {
            b.iter(|| find_hashmap(&sample))
        });
        c.bench_function(&format!("Radix {}", size), |b| {
            b.iter(|| find_radix(&sample))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
