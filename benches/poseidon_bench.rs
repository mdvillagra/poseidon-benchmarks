use criterion::{
    criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, SamplingMode, black_box
};
use std::time::Duration;

//dusk-network
use dusk_poseidon::sponge::hash as dusk_hash;
use dusk_bls12_381::BlsScalar as dusk_BlsScalar;

//lambdaworks
use hex_wrapper::Hex64;
use lambdaworks_crypto::hash::poseidon as lambda_poseidon;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381FieldElement;

//neptune
use blstrs::Scalar as FrNeptune;
use ff::Field;
use neptune::sponge::vanilla::*;
use rand_xorshift::XorShiftRng;
use typenum::U4;

//risc0
use risc0_core::field::{
    baby_bear::{BabyBear, BabyBearElem, BabyBearExtElem},
    Elem, ExtElem,
};
use risc0_zkp::core::hash::poseidon_254::{self, Poseidon254HashSuite};
use rand::prelude::*;

fn poseidon_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Poseidon");
    group.sampling_mode(SamplingMode::Flat);

    let n_inputs: u32 = 1; //number of inputs to try
    let n_elems: usize = 4; //number of elements per try

    //input vectors initialization
    let mut dusk_input: Vec<dusk_BlsScalar> = Vec::new();
    let mut lambda_input: Vec<BLS12381FieldElement> = Vec::new();
    let mut neptune_input: Vec<FrNeptune> = Vec::new();
    let mut risc0_input: Vec<BabyBearElem> = Vec::new();

    //rngs
    let dusk_rng = &mut StdRng::seed_from_u64(0xc10d);
    const TEST_SEED: [u8; 16] = [
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ];
    let mut neptune_rng = XorShiftRng::from_seed(TEST_SEED);

    for rounds in 0..n_inputs {
        for _i in 0..n_elems {
            //dusk-network input preparation
            dusk_input.push(dusk_BlsScalar::random(dusk_rng));
            //neptune input preparation
            neptune_input.push(FrNeptune::random(&mut neptune_rng));
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
            //risc0 input
            risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                &mut neptune_rng,
            )));
        }

        //Poseidon instantiations
        let lambda_pos = lambda_poseidon::Poseidon::new();
        let neptune_constants = Sponge::<FrNeptune, U4>::simplex_constants(n_elems);
        let mut neptune_sponge = Sponge::new_with_constants(&neptune_constants, Mode::Duplex);
        let acc = &mut (); //necesary for neptune
        let risc0_pos = Poseidon254HashSuite::new_suite();

        risc0_pos.hashfn.hash_elem_slice(&risc0_input);

        //dusk-network test
        group.bench_with_input(
            BenchmarkId::new("Dusk-Network", rounds as u32),
            &dusk_input,
            |b, dusk_input | b.iter(|| black_box(&dusk_input)),
        );

        //risc0 test
        group.bench_with_input(
            BenchmarkId::new("Risc0", rounds as u32),
            &risc0_input,
            |b, risc0_input| b.iter(|| black_box(risc0_pos.hashfn.hash_elem_slice(&risc0_input))),
        );

        //neptune test
        group.bench_with_input(
            BenchmarkId::new("Neptune", rounds as u32),
            &neptune_input,
            |b, neptune_input| {
                b.iter(|| black_box( {
                    neptune_sponge.absorb_elements(&neptune_input, acc).unwrap();
                    neptune_sponge.squeeze_elements(1, acc);
                }))
            },
        );

        //lambdaworks test
        group.bench_with_input(
            BenchmarkId::new("Lambdaworks", rounds as u32),
            &lambda_input[0..],
            |b, lambda_input| b.iter(|| black_box( lambda_pos.hash(&lambda_input[0..]))),
        );

        //group.significance_level(0.05).sample_size(100).measurement_time(Duration::from_secs(11));
    }
    group.finish();
}

criterion_group!(benches, poseidon_benchmark);
criterion_main!(benches);
