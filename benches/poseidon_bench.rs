use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use ark_std::UniformRand;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, BenchmarkGroup, SamplingMode};
use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use lambdaworks_math::field::traits::IsPrimeField;
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

use lambdaworks_crypto::hash::poseidon as lambda_poseidon;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381FieldElement;

use hex_wrapper::Hex64;

fn poseidon_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Poseidon");
    group.sampling_mode(SamplingMode::Flat);

    let n_inputs = 5; //number of inputs to try
    let n_elems = 4; //number of elements per try

    let mut ark_input: Vec<ark_Fr> = Vec::new();
    let mut dusk_input: Vec<dusk_BlsScalar> = Vec::new();
    let mut lambda_input: Vec<BLS12381FieldElement> = Vec::new();

    //Poseidon instantiations
    let ark_pos = ark_Poseidon::new();
    let lambda_pos = lambda_poseidon::Poseidon::new();

    for rounds in 0..n_inputs {
        let mut ark_rng = ark_std::test_rng();
        let dusk_rng = &mut StdRng::seed_from_u64(0xc10d);

        for _i in 0..n_elems {
            //arkworks input preparation
            ark_input.push(ark_Fr::rand(&mut ark_rng));
            //dusk-network input preparation
            dusk_input.push(dusk_BlsScalar::random(dusk_rng));
            //lambdaworks input preparation
            let hex_input = format!(
                "{}{}{}{}{}{}",
                Hex64::rand().to_string(),
                Hex64::rand().to_string(),
                Hex64::rand().to_string(),
                Hex64::rand().to_string(),
                Hex64::rand().to_string(),
                Hex64::rand().to_string()
            );
            let element = BLS12381FieldElement::from_hex(&hex_input[0..]).unwrap();
            lambda_input.push(element);
        }

        //arkworks test
        group.bench_with_input(
            BenchmarkId::new("arnaucube-arkworks", rounds as u32),
            &ark_input,
            |b, ark_input| b.iter(|| ark_pos.hash(ark_input.clone())),
        );

        //dusk-network test
        group.bench_with_input(
            BenchmarkId::new("Dusk-Network", rounds as u32),
            &dusk_input,
            |b, dusk_input| b.iter(|| dusk_hash(&dusk_input)),
        );

        //lambdaworks test
        group.bench_with_input(
            BenchmarkId::new("Lambdaworks", rounds as u32),
            &lambda_input[0..],
            |b, lambda_input| b.iter(|| lambda_pos.hash(&lambda_input[0..])),
        );

        //group.significance_level(0.05).sample_size(100).measurement_time(Duration::from_secs(11));
    }
    group.finish();
}

criterion_group!(benches, poseidon_benchmark);
criterion_main!(benches);
