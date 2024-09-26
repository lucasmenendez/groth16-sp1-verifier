mod utils;

fn main() {
    let vk_json = std::fs::read_to_string("./data/verification_key.json").unwrap();
    let proof_json = std::fs::read_to_string("./data/proof.json").unwrap();
    let public_inputs_json = std::fs::read_to_string("./data/public_inputs.json").unwrap();

    let is_valid = utils::verify_from_json(vk_json, proof_json, public_inputs_json);
    println!("Is valid: {:?}", is_valid);
}