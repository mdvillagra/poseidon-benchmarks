extern crate poseidon_functions;

use ark_ff::fields::{Fp128, Fp64, MontBackend, MontConfig, PrimeField};

#[derive(MontConfig)]
#[modulus = "11"]
#[generator = "2"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;

extern "C" {
    fn hello_world();
}

fn main() {
    let a = Fr::from(5);
    let b = Fr::from(6);
    println!("{:?}", <Fr as PrimeField>::MODULUS);
    println!("{}", a + b);
}
