use libc::uint64_t;
use std::ffi::c_void;
use std::os::raw::c_ulonglong;

mod sponge;

type felt_t = [c_ulonglong; 4];

#[link(name = "_pos", kind = "static")]
extern "C" {
    fn permutation_3(state: *mut felt_t);
}

fn main() {
    
    let mut zeroes: felt_t = [0, 0, 0, 0];
    let mut ones: felt_t = [1, 1, 1, 1];
    let mut state: [felt_t; 3] = [zeroes, zeroes, zeroes];

    println!("{:?}", state);

    unsafe {
        permutation_3(state[0..].as_mut_ptr());
    }

    println!("{:?}", state);
}
