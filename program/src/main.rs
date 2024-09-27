//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use groth16_verifier::verify_from_json;

pub fn main() {
    // read the verify_key, proof and public_inputs
    let str_vk = sp1_zkvm::io::read::<String>();
    let str_proof = sp1_zkvm::io::read::<String>();
    let str_public_inputs = sp1_zkvm::io::read::<String>();
    // verify the received proof
    let is_valid = verify_from_json(str_vk, str_proof, str_public_inputs);
    sp1_zkvm::io::commit(&is_valid);
}
