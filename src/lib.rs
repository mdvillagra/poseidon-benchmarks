mod poseidon_ark;
use ark_bn254::Fr;
use ark_std::str::FromStr;
use crate::poseidon_ark::lib::Poseidon;

//Calls the poseidon hash based on arkworks
//Input is a number in the scalar field F_r of BN254
//https://docs.rs/ark-bn254/latest/ark_bn254/
//r = 21888242871839275222246405745257275088548364400416034343698204186575808495617
pub fn poseidon_ark_hash(s: &str) -> String {
    let b: Fr = Fr::from_str(s).unwrap();
    let mut big_arr: Vec<Fr> = Vec::new();
    big_arr.push(b.clone());
    let poseidon = Poseidon::new();
    let h = poseidon.hash(big_arr.clone()).unwrap();
    h.to_string()
}