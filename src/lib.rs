//arkworks libraries
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
