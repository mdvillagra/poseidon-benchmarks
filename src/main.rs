mod poseidon_ark;
use ark_bn254::Fr;
use ark_std::str::FromStr;
use crate::poseidon_ark::lib::Poseidon;


fn main() {
    let b1: Fr = Fr::from_str(
        "0",
    )
    .unwrap();

    let mut big_arr: Vec<Fr> = Vec::new();
    big_arr.push(b1.clone());

    let poseidon = Poseidon::new();
    let h = poseidon.hash(big_arr.clone()).unwrap();

    println!("The hash is: {}", h.to_string());
}

