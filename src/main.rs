extern crate poseidon_functions;

extern "C" {
    fn hello_world();
}

fn main() {
    let cad = "402384238";

    let ark_h = poseidon_functions::poseidon_ark_hash(cad);
    println!("The arkworks poseidon hash is: {}", ark_h);

    let dusk_h = poseidon_functions::poseidon_dusk_hash();
    println!("The dusk poseidon hash is: {}", format!("{:#x}", dusk_h));

    unsafe {
        hello_world();
    }
}
