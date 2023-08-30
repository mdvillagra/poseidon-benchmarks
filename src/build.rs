use cc;

fn main(){
    cc::Build::new().file("./src/poseidon_ce.c").compile("poseidon_ce");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=static=poseidon_ce");
}