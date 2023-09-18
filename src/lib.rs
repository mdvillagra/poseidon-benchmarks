//arkworks libraries
mod poseidon_ark;
use crate::poseidon_ark::lib::Poseidon;
use ark_bn254::Fr;
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use ark_std::str::FromStr;

//dusk-network libraries
mod dusk_poseidon;
use dusk_poseidon::src::lib::sponge as dusk_sponge;
//use dusk_plonk::prelude::*;
use dusk_plonk::prelude::BlsScalar;

//other precompiles
//use sha2::Sha256;
//use ripemd::Ripemd160;

/*
Calls poseidon hash based on dusk-network
Input is a number in the scalar field of BLS12_381
r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
255 bits
*/
pub fn poseidon_dusk_hash() -> BlsScalar {
    let rng = &mut StdRng::seed_from_u64(0xc10d);

    let message = [
        BlsScalar::random(rng),
        BlsScalar::random(rng),
        BlsScalar::random(rng),
        BlsScalar::random(rng),
        BlsScalar::random(rng),
    ];

    //let cad = format!("{:#x}", BlsScalar::random(rng));

    dusk_sponge::hash(&message)
}

/*
Calls poseidon hash based on arkworks
Input is a number in the scalar field F_r of BN254
r = 21888242871839275222246405745257275088548364400416034343698204186575808495617
254 bits
https://docs.rs/ark-bn254/latest/ark_bn254/
*/
pub fn poseidon_ark_hash(s: &str) -> String {
    let b: Fr = Fr::from_str(s).unwrap();
    let mut big_arr: Vec<Fr> = Vec::new();
    big_arr.push(b.clone());
    let poseidon = Poseidon::new();
    let h = poseidon.hash(big_arr.clone()).unwrap();
    h.to_string()
}
