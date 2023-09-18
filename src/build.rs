use cc;

fn main(){
    cc::Build::new().file("src/poseidon_ce.c").compile("poseidon_ce");
}