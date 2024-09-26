use crate::utils::{json_to_g1, json_to_g1_vec, json_to_g2};

use ark_bn254::Bn254;
use ark_groth16::VerifyingKey;

use serde_json::Value;

pub fn from_json(path: String) -> VerifyingKey<Bn254> {
    let json_content = std::fs::read_to_string(path).unwrap();
    let json_value: Value = serde_json::from_str(&json_content).unwrap();

    let alpha_g1 = json_to_g1(&json_value, "vk_alpha_1");
    let beta_g2 = json_to_g2(&json_value, "vk_beta_2");
    let gamma_g2 = json_to_g2(&json_value, "vk_gamma_2");
    let delta_g2 = json_to_g2(&json_value, "vk_delta_2");
    let gamma_abc_g1 = json_to_g1_vec(&json_value, "IC");

    VerifyingKey {
        alpha_g1,
        beta_g2,
        gamma_g2,
        delta_g2,
        gamma_abc_g1,
    }
}