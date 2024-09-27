use ark_bn254::{Bn254, Fr, Fq, Fq2, G1Affine, G2Affine, G1Projective, G2Projective};
use ark_ff::BigInteger256;
use ark_groth16::{VerifyingKey, Proof, Groth16, prepare_verifying_key};

use num_bigint::BigUint;
use serde_json::Value;

use std::str::FromStr;

pub fn verify_from_json(vk_json: String, proof_json: String, public_inputs_json: String) -> bool {
     // decode verifycation key, proof and public inputs
    let vk = vk_from_json(vk_json);
    let proof = proof_from_json(proof_json);
    let public_inputs = public_inputs_from_json(public_inputs_json);
    // prepare verifying key
    let pvk = prepare_verifying_key(&vk);
    // verify proof
    let is_valid = Groth16::<Bn254>::verify_proof(&pvk, &proof, &public_inputs);
    is_valid.is_ok()
}

pub fn vk_from_json(json_content: String) -> VerifyingKey<Bn254> {
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

pub fn proof_from_json(json_content: String) -> Proof<Bn254> {
    // let json_content = std::fs::read_to_string(path).unwrap();
    let json_value: Value = serde_json::from_str(&json_content).unwrap();

    let a = json_to_g1(&json_value, "pi_a");
    let b = json_to_g2(&json_value, "pi_b");
    let c = json_to_g1(&json_value, "pi_c");

    Proof { a, b, c }
}

pub fn public_inputs_from_json(json_content: String) -> Vec<Fr> {
    let nums: Vec<String> = serde_json::from_str(&json_content).unwrap();
    let field_elements: Vec<Fr> = nums
        .iter()
        .map(|s| Fr::from_str(s).unwrap())
        .collect();

    field_elements
}

fn json_to_g1(json: &Value, key: &str) -> G1Affine {
    let els: Vec<String> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i.as_str().unwrap().to_string())
        .collect();
    G1Affine::from(G1Projective::new(
        fq_from_str(&els[0]),
        fq_from_str(&els[1]),
        fq_from_str(&els[2]),
    ))
}

fn json_to_g1_vec(json: &Value, key: &str) -> Vec<G1Affine> {
    let els: Vec<Vec<String>> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            i.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    els.iter()
        .map(|coords| {
            G1Affine::from(G1Projective::new(
                fq_from_str(&coords[0]),
                fq_from_str(&coords[1]),
                fq_from_str(&coords[2]),
            ))
        })
        .collect()
}

fn json_to_g2(json: &Value, key: &str) -> G2Affine {
    let els: Vec<Vec<String>> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            i.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let x = Fq2::new(fq_from_str(&els[0][0]), fq_from_str(&els[0][1]));
    let y = Fq2::new(fq_from_str(&els[1][0]), fq_from_str(&els[1][1]));
    let z = Fq2::new(fq_from_str(&els[2][0]), fq_from_str(&els[2][1]));
    G2Affine::from(G2Projective::new(x, y, z))
}

fn fq_from_str(s: &str) -> Fq {
    BigInteger256::try_from(BigUint::from_str(s).unwrap())
        .unwrap()
        .into()
}