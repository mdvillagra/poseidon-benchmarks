use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
    SamplingMode,
};
use std::time::Duration;

//sponge for cryptoexperts permutation
mod sponge;
use rand::Rng;
use sponge::*;

//dusk-network
use dusk_bls12_381::BlsScalar as dusk_BlsScalar;
use dusk_poseidon::sponge::hash as dusk_hash;

//neptune
use blstrs::Scalar as FrNeptune;
use ff::Field;
use neptune::sponge::vanilla::*;
use rand_xorshift::XorShiftRng;
use typenum::{U3, U4, U5, U9};

//risc0
use rand::prelude::*;
use risc0_core::field::{
    baby_bear::{BabyBear, BabyBearElem, BabyBearExtElem},
    Elem, ExtElem,
};
use risc0_zkp::core::hash::poseidon::PoseidonHashSuite;
use risc0_zkp::core::hash::poseidon_254::{self, Poseidon254HashSuite};

/**************************************************************
 * Poseidon benchmark with default parameters
**************************************************************/
fn poseidon_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Poseidon-all");
    group.sampling_mode(SamplingMode::Linear);

    let n_inputs: u32 = 32; //number of inputs to try
    let n_elems: usize = 7; //number of elements per try

    //input vectors initialization
    let mut dusk_input: Vec<dusk_BlsScalar> = Vec::new();
    let mut neptune_input: Vec<FrNeptune> = Vec::new();
    let mut risc0_input: Vec<BabyBearElem> = Vec::new();
    let mut risc0babybear_input: Vec<BabyBearElem> = Vec::new();
    let mut cryptoexperts_input: Vec<felt_t> = Vec::new();

    //rngs
    let dusk_rng = &mut StdRng::seed_from_u64(0xc10d);
    const TEST_SEED: [u8; 16] = [
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ];
    let mut neptune_rng = XorShiftRng::from_seed(TEST_SEED);
    let mut ce_rng = rand::thread_rng();

    for _i in 0..n_elems {
        //risc0 input
        risc0babybear_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
            &mut neptune_rng,
        )));
    }

    for rounds in 7..n_inputs {
        //Poseidon instantiations
        let neptune_constants = Sponge::<FrNeptune, U3>::simplex_constants(n_elems);
        let mut neptune_sponge = Sponge::new_with_constants(&neptune_constants, Mode::Simplex);
        let acc = &mut (); //necesary for neptune
        let risc0_pos = Poseidon254HashSuite::new_suite();
        let risc0babybear_pos = PoseidonHashSuite::new_suite();
        let ce_input_copy = cryptoexperts_input.clone();

        //cryptoexperts test
        if rounds == 7 || rounds == 15 || rounds == 23 || rounds == 31 {
            if rounds == 7 {
                //one field element
                cryptoexperts_input.clear();
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
            } else if rounds == 15 {
                //two field elements
                cryptoexperts_input.clear();
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
            } else if rounds == 23 {
                //three field elements
                cryptoexperts_input.clear();
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
            } else if rounds == 31 {
                //four field elements
                cryptoexperts_input.clear();
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
                cryptoexperts_input.push([
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                    ce_rng.gen::<u64>(),
                ]);
            }
            let ce_input_copy = cryptoexperts_input.clone();
            group.bench_with_input(
                BenchmarkId::new("Cryptoexperts", rounds as u32),
                &ce_input_copy,
                |b, ce_input_copy| b.iter(|| black_box(hash3(&ce_input_copy, 3))),
            );
        }

        //risc0 input with babybear
        risc0babybear_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
            &mut neptune_rng,
        )));
        group.bench_with_input(
            BenchmarkId::new("Risc0_Babybear", rounds as u32),
            &risc0babybear_input,
            |b, risc0babybear_input| {
                b.iter(|| {
                    black_box(
                        risc0babybear_pos
                            .hashfn
                            .hash_elem_slice(&risc0babybear_input),
                    )
                })
            },
        );

        //risc0 test
        if rounds == 7 || rounds == 15 || rounds == 23 || rounds == 31 {
            if rounds == 7 {
                //one field elements
                risc0_input.clear();
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
            } else if rounds == 15 {
                // two field elements
                risc0_input.clear();
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
            } else if rounds == 23 {
                //three field elements
                risc0_input.clear();
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
            } else if rounds == 31 {
                //four field elements
                risc0_input.clear();
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
                risc0_input.push(BabyBearElem::from(XorShiftRng::gen::<u32>(
                    &mut neptune_rng,
                )));
            }
            group.bench_with_input(
                BenchmarkId::new("Risc0", rounds as u32),
                &risc0_input,
                |b, risc0_input| {
                    b.iter(|| black_box(risc0_pos.hashfn.hash_elem_slice(&risc0_input)))
                },
            );
        }

        //dusk-network test
        if rounds == 7 || rounds == 15 || rounds == 23 || rounds == 31 {
            if rounds == 7 {
                //one field element
                dusk_input.clear();
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
            } else if rounds == 15 {
                //two field elements
                dusk_input.clear();
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
            } else if rounds == 23 {
                //three field elements
                dusk_input.clear();
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
            } else if rounds == 31 {
                //four field elements
                dusk_input.clear();
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
                dusk_input.push(dusk_BlsScalar::random(dusk_rng));
            }
            group.bench_with_input(
                BenchmarkId::new("Dusk-Network", rounds as u32),
                &dusk_input,
                |b, dusk_input| b.iter(|| black_box(dusk_hash(&dusk_input))),
            );
        }

        //neptune test
        if rounds == 7 || rounds == 15 || rounds == 23 || rounds == 31 {
            if rounds == 7 {
                //one field elements
                neptune_input.clear();
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
            } else if rounds == 15 {
                // two field elements
                neptune_input.clear();
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
            } else if rounds == 23 {
                //three field elements
                neptune_input.clear();
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
            } else if rounds == 31 {
                //four field elements
                neptune_input.clear();
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
                neptune_input.push(FrNeptune::random(&mut neptune_rng));
            }
            group.bench_with_input(
                BenchmarkId::new("Neptune", rounds as u32),
                &neptune_input,
                |b, neptune_input| {
                    b.iter(|| {
                        black_box({
                            neptune_sponge.absorb_elements(&neptune_input, acc).unwrap();
                            //neptune_sponge.squeeze_elements(1, acc);
                        })
                    })
                },
            );
        }
    }
    //group.significance_level(0.05).sample_size(100).measurement_time(Duration::from_secs(11));
    group.finish();
}

/**************************************************************
 * Neptune benchmark with different widths
**************************************************************/
fn neptune_widths_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Neptune-widths");
    group.sampling_mode(SamplingMode::Linear);

    let n_inputs: u32 = 4; //number of inputs to try
    let n_elems: usize = 1; //number of elements per try

    //input vectors initialization
    let mut neptune_input: Vec<FrNeptune> = Vec::new();

    //rngs
    const TEST_SEED: [u8; 16] = [
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ];
    let mut neptune_rng = XorShiftRng::from_seed(TEST_SEED);

    for rounds in 0..n_inputs {
        for _i in 0..n_elems {
            //neptune input preparation
            neptune_input.push(FrNeptune::random(&mut neptune_rng));
        }

        //Poseidon instantiations
        let neptune_constants_U3 = Sponge::<FrNeptune, U3>::simplex_constants(n_elems);
        let neptune_constants_U4 = Sponge::<FrNeptune, U4>::simplex_constants(n_elems);
        let neptune_constants_U5 = Sponge::<FrNeptune, U5>::simplex_constants(n_elems);
        let neptune_constants_U9 = Sponge::<FrNeptune, U9>::simplex_constants(n_elems);
        let mut neptune_sponge_U3 =
            Sponge::new_with_constants(&neptune_constants_U3, Mode::Simplex);
        let mut neptune_sponge_U4 =
            Sponge::new_with_constants(&neptune_constants_U4, Mode::Simplex);
        let mut neptune_sponge_U5 =
            Sponge::new_with_constants(&neptune_constants_U5, Mode::Simplex);
        let mut neptune_sponge_U9 =
            Sponge::new_with_constants(&neptune_constants_U9, Mode::Simplex);
        let acc = &mut (); //necesary for neptune

        //neptune test with U3
        group.bench_with_input(
            BenchmarkId::new("Neptune-Width3", rounds as u32),
            &neptune_input,
            |b, neptune_input| {
                b.iter(|| {
                    black_box({
                        neptune_sponge_U3
                            .absorb_elements(&neptune_input, acc)
                            .unwrap();
                        //neptune_sponge.squeeze_elements(1, acc);
                    })
                })
            },
        );

        //neptune test with U4
        group.bench_with_input(
            BenchmarkId::new("Neptune-Width4", rounds as u32),
            &neptune_input,
            |b, neptune_input| {
                b.iter(|| {
                    black_box({
                        neptune_sponge_U4
                            .absorb_elements(&neptune_input, acc)
                            .unwrap();
                        //neptune_sponge.squeeze_elements(1, acc);
                    })
                })
            },
        );

        //neptune test with U5
        group.bench_with_input(
            BenchmarkId::new("Neptune-Width5", rounds as u32),
            &neptune_input,
            |b, neptune_input| {
                b.iter(|| {
                    black_box({
                        neptune_sponge_U5
                            .absorb_elements(&neptune_input, acc)
                            .unwrap();
                        //neptune_sponge.squeeze_elements(1, acc);
                    })
                })
            },
        );

        //neptune test with U9
        group.bench_with_input(
            BenchmarkId::new("Neptune-Width9", rounds as u32),
            &neptune_input,
            |b, neptune_input| {
                b.iter(|| {
                    black_box({
                        neptune_sponge_U9
                            .absorb_elements(&neptune_input, acc)
                            .unwrap();
                        //neptune_sponge.squeeze_elements(1, acc);
                    })
                })
            },
        );
    }
    //group.significance_level(0.05).sample_size(100).measurement_time(Duration::from_secs(11));
    group.finish();
}

/**************************************************************
 * Cryptoexperts benchmark with different widths
**************************************************************/
fn cryptoxperts_widths_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Cryptoexpert-widths");
    group.sampling_mode(SamplingMode::Flat);

    let n_inputs: u32 = 4; //number of inputs to try
    let n_elems: usize = 1; //number of elements per try

    let mut cryptoexperts_input: Vec<felt_t> = Vec::new();

    //rng
    let mut ce_rng = rand::thread_rng();

    //cryptoexperts test
    for rounds in 0..n_inputs {
        for _i in 0..n_elems {
            //cryptoexperts input preparation
            //one field element
            cryptoexperts_input.push([
                ce_rng.gen::<u64>(),
                ce_rng.gen::<u64>(),
                ce_rng.gen::<u64>(),
                ce_rng.gen::<u64>(),
            ]);
        }

        let ce_input_copy = cryptoexperts_input.clone();
        group.bench_with_input(
            BenchmarkId::new("Cryptoexperts-width3", rounds as u32),
            &ce_input_copy,
            |b, ce_input_copy| b.iter(|| black_box(hash3(&ce_input_copy, 3))),
        );

        let ce_input_copy = cryptoexperts_input.clone();
        group.bench_with_input(
            BenchmarkId::new("Cryptoexperts-width4", rounds as u32),
            &ce_input_copy,
            |b, ce_input_copy| b.iter(|| black_box(hash4(&ce_input_copy))),
        );

        let ce_input_copy = cryptoexperts_input.clone();
        group.bench_with_input(
            BenchmarkId::new("Cryptoexperts-width5", rounds as u32),
            &ce_input_copy,
            |b, ce_input_copy| b.iter(|| black_box(hash5(&ce_input_copy))),
        );

        let ce_input_copy = cryptoexperts_input.clone();
        group.bench_with_input(
            BenchmarkId::new("Cryptoexperts-width9", rounds as u32),
            &ce_input_copy,
            |b, ce_input_copy| b.iter(|| black_box(hash9(&ce_input_copy))),
        );
    }

    //group.significance_level(0.05).sample_size(100).measurement_time(Duration::from_secs(11));
    group.finish();
}

criterion_group!(benches, poseidon_benchmark);
criterion_group!(benches, neptune_widths_bench);
criterion_group!(benches, cryptoxperts_widths_bench);
criterion_main!(benches);
