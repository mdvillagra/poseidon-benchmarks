//#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

//sponge for cryptoexperts permutation
#[path = "../benches/sponge.rs"]
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

fn main() {
    //#[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    dusk();

    drop(_profiler);

}

/**************************************************************
 * Poseidon benchmark with default parameters
**************************************************************/
fn dusk() {
    

    let n_inputs: u32 = 32; //number of inputs to try
    let n_elems: usize = 7; //number of elements per try

    //input vectors initialization
    let mut dusk_input: Vec<dusk_BlsScalar> = Vec::new();

    //rngs
    let dusk_rng = &mut StdRng::seed_from_u64(0xc10d);
    const TEST_SEED: [u8; 16] = [
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ];

    for rounds in 7..n_inputs {
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

            dusk_hash(&dusk_input);
        }
    }
}
