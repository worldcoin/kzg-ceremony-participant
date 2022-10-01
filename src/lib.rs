#[cfg(target_family = "wasm")]
mod wasm;

use eyre::Result;
use kzg_ceremony_crypto::CeremonyError;
use kzg_ceremony_crypto::Contribution;
use kzg_ceremony_crypto::BLST;
use rand::Rng;
use rayon::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::io::Write;
use std::{fs::File, path::Path};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ContributionJson {
    pub contributions: Vec<Contribution>,
}

fn load_json_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_json_file(path: &Path, contents: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn contribute_with_json_file(entropy: [u8; 32], in_path: &Path, out_path: &Path) -> Result<()> {
    let json = load_json_file(in_path)?;
    let result = contribute_with_json_string(entropy, json)?;
    write_json_file(out_path, &result)
}

pub fn contribute_with_json_string(entropy: [u8; 32], json: String) -> Result<String> {
    let contributions = serde_json::from_str::<ContributionJson>(&json)?;
    let contributions = contribute(entropy, contributions.contributions)?;
    Ok(serde_json::to_string(&contributions)?)
}

pub fn contribute(
    entropy: [u8; 32],
    mut contributions: Vec<kzg_ceremony_crypto::Contribution>,
) -> Result<Vec<kzg_ceremony_crypto::Contribution>> {
    contributions
        .par_iter_mut()
        .map(|contribution| {
            contribution.add_entropy::<BLST>(entropy)?;
            Ok(())
        })
        .collect::<Result<Vec<_>, CeremonyError>>()?;
    Ok(contributions)
}

pub fn get_entropy(seed: &[u8]) -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let entropy: [u8; 32] = rng.gen();
    let mut buf = [0u8; 32];

    buf.copy_from_slice(
        &Sha256::new()
            .chain_update(seed)
            .chain_update(entropy)
            .finalize(),
    );

    buf
}
