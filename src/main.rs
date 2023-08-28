extern crate poseidon_functions;

mod dusk_poseidon;
use dusk_poseidon::src::lib::sponge as dusk_sponge;
use dusk_plonk::prelude::*;

use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;



fn main() {
    let cad = "402384238";

    let ark_h = poseidon_functions::poseidon_ark_hash(cad);
    println!("The arkworks poseidon hash is: {}", ark_h);

    let dusk_h = poseidon_functions::poseidon_dusk_hash();
    println!("The dusk poseidon hash is: {}", format!("{:#x}", dusk_h));


}


