use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use ark_std::UniformRand;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
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
    let n_inputs = 2;

    let mut ark_input: Vec<ark_Fr> = Vec::new();
    let mut dusk_input: Vec<dusk_BlsScalar> = Vec::new();

    for rounds in 0..n_inputs {
        //arkworks input preparation
        let mut rng = ark_std::test_rng();
        for _i in 0..5 {
            ark_input.push(ark_Fr::rand(&mut rng));
        }
        let ark_pos = ark_Poseidon::new();

        //dusk-network input preparation
        let rng = &mut StdRng::seed_from_u64(0xc10d);
        for _i in 0..5 {
            dusk_input.push(dusk_BlsScalar::random(rng));
        }

        //arkworks test
        group.bench_with_input(
            BenchmarkId::new("Arkworks", rounds as u32),
            &ark_input,
            |b, ark_input| b.iter(|| ark_pos.hash(ark_input.clone())),
        );

        //dusk-network test
        group.bench_with_input(
            BenchmarkId::new("Dusk-Network", rounds as u32),
            &dusk_input,
            |b, dusk_input| b.iter(|| dusk_hash(&dusk_input)),
        );

        //group.significance_level(0.05).sample_size(100).measurement_time(Duration::from_secs(11));
    }
    group.finish();
}

criterion_group!(benches, poseidon_benchmark);
criterion_main!(benches);
