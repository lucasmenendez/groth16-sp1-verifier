mod utils;
mod vk;
mod proof;
mod inputs;

use ark_bn254::Bn254;
use ark_groth16::{Groth16, prepare_verifying_key};

fn main() {
    let vk = vk::from_json("./data/verification_key.json".to_string());
    println!("{:?}", vk);

    let proof = proof::from_json("./data/proof.json".to_string());
    println!("{:?}", proof);

    let public_inputs = inputs::from_json("./data/public_inputs.json".to_string());
    println!("{:?}", public_inputs);

    let pvk = prepare_verifying_key(&vk);
    let is_valid = Groth16::<Bn254>::verify_proof(&pvk, &proof, &public_inputs);
    println!("Is valid: {:?}", is_valid);
}