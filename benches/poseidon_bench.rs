use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use ark_std::UniformRand;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use poseidon_functions::poseidon_ark_hash;
use std::time::Duration;

#[path = "../src/poseidon_ark/mod.rs"]
mod poseidon_ark;
use ark_bn254::Fr as ark_Fr;
use poseidon_ark::lib::Poseidon as ark_Poseidon;

#[path = "../src/dusk_poseidon/mod.rs"]
mod dusk_poseidon;
use dusk_plonk::prelude::BlsScalar as dusk_BlsScalar;
use dusk_poseidon::src::lib::sponge::hash as dusk_hash;

//#[path="../src/poseidon_rust/src/lib.rs"]
//mod poseidon_rust;

fn poseidon_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Poseidon");

    //dusk initialization
    let rng = &mut StdRng::seed_from_u64(0xc10d);
    let mut dusk_input: Vec<dusk_BlsScalar> = Vec::new();
    dusk_input.clear();
    for _i in 0..5 {
        dusk_input.push(dusk_BlsScalar::random(rng));
    }

    for rounds in 0..3 {
        //ark initialization
        let mut ark_input: Vec<ark_Fr> = Vec::new();
        let mut rng = ark_std::test_rng();
        ark_input.clear();
        for _i in 0..5 {
            ark_input.push(ark_Fr::rand(&mut rng));
        }

        let a_pos = ark_Poseidon::new();

        //group
        //    .significance_level(0.05)
        //    .sample_size(100)
        //    .measurement_time(Duration::from_secs(11));
        //group.bench_function("Ark Hash", |b| b.iter(|| a_pos.hash(ark_input.clone())));
        group.bench_with_input(
            BenchmarkId::new("Ark Hash", rounds as u32),
            &ark_input,
            |b, ark_input| b.iter(|| a_pos.hash(ark_input.clone())),
        );

        //group.significance_level(0.05).sample_size(100).measurement_time(Duration::from_secs(11));
        //group.bench_function("Dusk Hash", |b| b.iter(|| dusk_hash(&dusk_input)));
    }
    group.finish();
}

criterion_group!(benches, poseidon_benchmark);
criterion_main!(benches);
