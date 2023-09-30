use libc::uint64_t;
use std::ffi::c_void;
use std::os::raw::c_ulonglong;

#[link(name = "_pos", kind = "static")]
extern "C" {
    fn permutation_3(state: *mut c_ulonglong);
}

fn main() {
    
    let mut state: [c_ulonglong; 4] = [0,0,0,0];

    println!("{:?}", state);
    
    unsafe {
        permutation_3(state[0..].as_mut_ptr());
    }

    println!("{:?}", state);
    
}
