use std::os::raw::c_ulonglong;

pub type felt_t = [c_ulonglong; 4];

#[link(name = "_pos", kind = "static")]
extern "C" {
    fn permutation_3(state: *mut felt_t);
    fn permutation_4(state: *mut felt_t);
    fn permutation_5(state: *mut felt_t);
    fn permutation_9(state: *mut felt_t);
}

/*********************************************************
Hashing function for width 9
*********************************************************/
pub fn hash9(input: &Vec<felt_t>) -> felt_t {
    let mut state = absorb9(input, 9);
    //squeeze(&mut state, r)
    state.pop().unwrap()
}

/*********************************************************
Absorbing stage for width 9
input is the unpadded input
r is the rate
*********************************************************/
fn absorb9(input: &Vec<felt_t>, r: usize) -> Vec<felt_t> {
    let mut state: Vec<felt_t> = Vec::new();
    let padded_input = pad(input, r as u32);

    init_state(&mut state, r);

    for i in (0..padded_input.len()).step_by(r) {
        add_block(&padded_input[i..i + r], &mut state, r);
        unsafe {
            permutation_9(state[0..].as_mut_ptr());
        }
    }
    state
}

/*********************************************************
Hashing function for width 5
*********************************************************/
pub fn hash5(input: &Vec<felt_t>) -> felt_t {
    let mut state = absorb5(input, 5);
    //squeeze(&mut state, r)
    state.pop().unwrap()
}

/*********************************************************
Absorbing stage for width 5
input is the unpadded input
r is the rate
*********************************************************/
fn absorb5(input: &Vec<felt_t>, r: usize) -> Vec<felt_t> {
    let mut state: Vec<felt_t> = Vec::new();
    let padded_input = pad(input, r as u32);

    init_state(&mut state, r);

    for i in (0..padded_input.len()).step_by(r) {
        add_block(&padded_input[i..i + r], &mut state, r);
        unsafe {
            permutation_5(state[0..].as_mut_ptr());
        }
    }
    state
}

/*********************************************************
Hashing function for width 4
*********************************************************/
pub fn hash4(input: &Vec<felt_t>) -> felt_t {
    let mut state = absorb4(input, 4);
    //squeeze(&mut state, r)
    state.pop().unwrap()
}

/*********************************************************
Absorbing stage for width 4
input is the unpadded input
r is the rate
*********************************************************/
fn absorb4(input: &Vec<felt_t>, r: usize) -> Vec<felt_t> {
    let mut state: Vec<felt_t> = Vec::new();
    let padded_input = pad(input, r as u32);

    init_state(&mut state, r);

    for i in (0..padded_input.len()).step_by(r) {
        add_block(&padded_input[i..i + r], &mut state, r);
        unsafe {
            permutation_4(state[0..].as_mut_ptr());
        }
    }
    state
}

/*********************************************************
Hashing function for width 3
*********************************************************/
pub fn hash(input: &Vec<felt_t>, r: usize) -> felt_t {
    let mut state = absorb(input, r);
    //squeeze(&mut state, r)
    state.pop().unwrap()
}

/*********************************************************
Squeezing stage for width 3
*********************************************************/
fn squeeze(state: &mut Vec<felt_t>, r: usize) -> felt_t {
    let mut output: Vec<felt_t> = Vec::new();

    output.extend_from_slice(&state[..r]);
    unsafe {
        permutation_3(state[0..].as_mut_ptr());
    }
    output.pop().unwrap()
}

/*********************************************************
Absorbing stage for width 3
input is the unpadded input
r is the rate
*********************************************************/
fn absorb(input: &Vec<felt_t>, r: usize) -> Vec<felt_t> {
    let mut state: Vec<felt_t> = Vec::new();
    let padded_input = pad(input, r as u32);

    init_state(&mut state, 3);

    for i in (0..padded_input.len()).step_by(r) {
        add_block(&padded_input[i..i + r], &mut state, r);
        unsafe {
            permutation_3(state[0..].as_mut_ptr());
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
fn add_block(input: &[felt_t], state: &mut Vec<felt_t>, r: usize) {
    for i in 0..r {
        for j in 0..4 {
            state[i][j] = state[i][j].wrapping_add(input[i][j]);
        }
    }
}

/*********************************************************
Padding function for an input vector.
The functions pads input with 0s and returns a vector
that is a multiple of r. If the length of the input is a
multiple of r, then no padding takes place.
*********************************************************/
fn pad(input: &Vec<felt_t>, r: u32) -> Vec<felt_t> {
    let mut padded_input: Vec<felt_t> = input.to_vec();

    while padded_input.len() as u32 % r != 0 {
        padded_input.push([0, 0, 0, 0]);
    }

    padded_input
}

/*********************************************************
Initialize a state vector
**********************************************************/
fn init_state(state: &mut Vec<felt_t>, t: usize) {
    state.clear();
    for _i in 0..t {
        state.push([0, 0, 0, 0]);
    }
}
