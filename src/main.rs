extern crate poseidon_functions;

extern "C" {
    fn hello_world();
}

fn main() {
    let cad = "402384238";


    let dusk_h = poseidon_functions::poseidon_dusk_hash();
    println!("The dusk poseidon hash is: {}", format!("{:#x}", dusk_h));

    unsafe {
        hello_world();
    }
}
