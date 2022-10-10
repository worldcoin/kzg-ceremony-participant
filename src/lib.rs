#[cfg(target_family = "wasm")]
mod wasm;

use eyre::Result;
use kzg_ceremony_crypto::BLST;
use kzg_ceremony_crypto::BatchContribution;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::io::Write;
use std::{fs::File, path::Path};

/// Called from the wasm bindings.
/// Calls the sequencer crypto library to do the actual work.
pub fn contribute_with_string(entropy: [u8; 32], json: String) -> Result<String> {
    let mut contribution = serde_json::from_str::<BatchContribution>(&json)?;
    contribution.add_entropy::<BLST>(&entropy.into())?;
    Ok(serde_json::to_string(&contribution)?)
}

/// Called from the wasm bindings.
/// Hashes the user provided entropy to 32 bytes.
pub fn hash_entropy(seed: &[u8]) -> Result<[u8; 32]> {
    // We add additional local randomness to the user provided seed.
    // This is using getrandom, which has for wasm32‑*‑unknown the browser crypto API implemented.
    // More information: https://docs.rs/getrandom/latest/getrandom
    let mut rng = rand::thread_rng();
    let entropy: [u8; 32] = rng.gen();

    let mut buf = [0u8; 32];
    buf.copy_from_slice(
        &Sha256::new()
            .chain_update(seed)
            .chain_update(entropy)
            .finalize(),
    );

    Ok(buf)
}

/// This is only used for local testing.
pub fn contribute_with_file(entropy: [u8; 32], in_path: &Path, out_path: &Path) -> Result<()> {
    // read from local file
    let mut file = File::open(in_path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    // contribute
    let result = contribute_with_string(entropy, json)?;

    // write result
    let mut file = File::create(out_path)?;
    file.write_all(result.as_bytes())?;
    Ok(())
}