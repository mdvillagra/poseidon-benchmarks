use std::os::raw::c_ulonglong;

#[link(name = "_pos", kind = "static")]
extern "C" {
    fn permutation_3(state: *mut c_ulonglong);
}

/*********************************************************
Hashing function
*********************************************************/
pub fn hash(input: &mut Vec<c_ulonglong>, r: usize) -> u64 {
    let mut state = absorb(input, r);
    squeeze(&mut state, r)
}

/*********************************************************
Squeezing stage
*********************************************************/
fn squeeze(state: &mut Vec<c_ulonglong>, r: usize) -> c_ulonglong {
    let mut output: Vec<c_ulonglong> = Vec::new();

    output.extend_from_slice(&state[..r]);
    //add permutation HERE
    output.pop().unwrap()
}

/*********************************************************
Absorbing stage
input is the unpadded input
r is the rate
*********************************************************/
fn absorb(input: &mut Vec<c_ulonglong>, r: usize) -> Vec<u64> {
    let mut state: Vec<c_ulonglong> = Vec::new();
    let padded_input = pad(input, r as u32);

    init_state(&mut state, 3);

    for i in (0..padded_input.len()).step_by(r) {
        add_block(&padded_input[i..i + r], &mut state, r);
        let mut state_c: [c_ulonglong; 3] = state.clone().try_into().unwrap();
        //add permutation HERE
        unsafe {
            permutation_3(state_c[0..].as_mut_ptr());
        }
    }
    state
}

/*********************************************************
Adds the inner state with the input slice
intput is an input slice
state is the current state of the sponge
r is the rate
*********************************************************/
fn add_block(input: &[c_ulonglong], state: &mut Vec<c_ulonglong>, r: usize) {
    for i in 0..r {
        state[i] = state[i] + input[i];
    }
}

/*********************************************************
Padding function for an input vector.
The functions pads input with 0s and returns a vector
that is a multiple of r. If the length of the input is a
multiple of r, then no padding takes place.
*********************************************************/
fn pad(input: &Vec<c_ulonglong>, r: u32) -> Vec<c_ulonglong> {
    let mut padded_input: Vec<c_ulonglong> = input.to_vec();

    while padded_input.len() as u32 % r != 0 {
        padded_input.push(0);
    }

    padded_input
}

/*********************************************************
Initialize a state vector
**********************************************************/
fn init_state(state: &mut Vec<c_ulonglong>, t: usize) {
    state.clear();
    for _i in 0..t {
        state.push(0);
    }
}
