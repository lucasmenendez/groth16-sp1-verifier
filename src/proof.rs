use crate::utils::{json_to_g1, json_to_g2};

use ark_bn254::Bn254;
use ark_groth16::Proof;

use serde_json::Value;

pub fn from_json(path: String) -> Proof<Bn254> {
    let json_content = std::fs::read_to_string(path).unwrap();
    let json_value: Value = serde_json::from_str(&json_content).unwrap();

    let a = json_to_g1(&json_value, "pi_a");
    let b = json_to_g2(&json_value, "pi_b");
    let c = json_to_g1(&json_value, "pi_c");

    Proof { a, b, c }
}