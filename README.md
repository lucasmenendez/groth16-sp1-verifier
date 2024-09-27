# SP1 Groth16 Verifier

Basic Groth16 SnarkJS verifier compatible with SP1. It uses [arkworks-rs/groth16](https://github.com/arkworks-rs/groth16).

This project has been developed with the [SP1](https://github.com/succinctlabs/sp1) template for creating an end-to-end project
that can generate a proof of any RISC-V program.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html)

## Running the Project

There are four main ways to run this project: build a program, execute a program, generate a core proof, and
generate an EVM-compatible proof.

### Build the Program

To build the program, run the following command:

```sh
cd program
cargo prove build
```

### Execute the Program

To run the program without generating a proof:

```sh
cd script
cargo run --release -- --execute --vk <path_to_verify_key> --proof <path_to_proof> --inputs <path_to_public_inputs>
```

This will execute the program and display the output.

### Generate a Core Proof

To generate a core proof for your program:

```sh
cd script
cargo run --release -- --prove --vk <path_to_verify_key> --proof <path_to_proof> --inputs <path_to_public_inputs>
```

### Generate an EVM-Compatible Proof

> [!WARNING]
> You will need at least 128GB RAM to generate a Groth16 or PLONK proof.

To generate a proof that is small enough to be verified on-chain and verifiable by the EVM:

```sh
cd script
cargo run --release --bin evm -- --system groth16 --vk <path_to_verify_key> --proof <path_to_proof> --inputs <path_to_public_inputs>
```

this will generate a Groth16 proof. If you want to generate a PLONK proof, run the following command:

```sh
cargo run --release --bin evm -- --system plonk --vk <path_to_verify_key> --proof <path_to_proof> --inputs <path_to_public_inputs>
```

These commands will also generate fixtures that can be used to test the verification of SP1 zkVM proofs
inside Solidity.

### Retrieve the Verification Key

To retrieve your `programVKey` for your on-chain contract, run the following command:

```sh
cargo prove vkey --elf elf/riscv32im-succinct-zkvm-elf
```

## Using the Prover Network

We highly recommend using the Succinct prover network for any non-trivial programs or benchmarking purposes. For more information, see the [setup guide](https://docs.succinct.xyz/generating-proofs/prover-network.html).

To get started, copy the example environment file:

```sh
cp .env.example .env
```

Then, set the `SP1_PROVER` environment variable to `network` and set the `SP1_PRIVATE_KEY`
environment variable to your whitelisted private key.

For example, to generate an EVM-compatible proof using the prover network, run the following
command:

```sh
SP1_PROVER=network SP1_PRIVATE_KEY=... cargo run --release --bin evm --vk <path_to_verify_key> --proof <path_to_proof> --inputs <path_to_public_inputs>
```