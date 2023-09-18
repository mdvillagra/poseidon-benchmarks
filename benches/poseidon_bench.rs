use criterion::{criterion_group, criterion_main, Criterion};
use poseidon_functions::poseidon_ark_hash;
use std::time::Duration;

#[path="../src/poseidon_ark/mod.rs"]
mod poseidon_ark;

#[path="../src/dusk_poseidon/mod.rs"]
mod dusk_poseidon;

#[path="../src/poseidon_rust/src/lib.rs"]
mod poseidon_rust;

fn criterion_benchmark(c: &mut Criterion){
    let b1 = "12242166908188651009877250812424843524687801523336557272219921456462821518061";

    let mut group = c.benchmark_group("poseidon-ark");
    group.significance_level(0.05).sample_size(100).measurement_time(Duration::from_secs(25));
    group.bench_function("hash", |b| {
        b.iter(|| poseidon_ark_hash(b1))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);