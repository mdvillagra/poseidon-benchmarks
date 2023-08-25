extern crate poseidon_functions;

fn main() {
    let cad = "402384238";

    let new_h = poseidon_functions::poseidon_ark_hash(cad);
    println!("The new hash is: {}", new_h);
}


