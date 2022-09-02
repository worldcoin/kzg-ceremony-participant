mod contribution;
use std::{fs::File, path::Path};
use std::panic;

use ark_bls12_381::{Fr as ScalarField, G1Affine, G2Affine};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Field;
use ark_serialize::Read;
use ark_serialize::Write;
use ark_std::UniformRand;
use eyre::Result;
use js_sys::Promise;
use rand::thread_rng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_rayon::{init_thread_pool};

use crate::contribution::*;

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

/**
 * We'll use this function for the cli
 */
pub fn contribute_with_file(in_path: &Path, out_path: &Path) -> Result<()> {
    let json = load_json_file(in_path)?;
    let result = contribute_with_string(json)?;
    write_json_file(out_path, &result)
}

/**
 * We'll use this function in the wasm
 */
pub fn contribute_with_string(json: String) -> Result<String> {
    let prev = serde_json::from_str(&json).unwrap();
    let post = contribute(serde_json::from_value::<Contributions>(prev)?)?;
    Ok(serde_json::to_string(&post)?)
}

/**
 * Apply a user's contibution to the setup
 */
fn contribute(prev_contributions: Contributions) -> Result<Contributions> {
    let mut rng = thread_rng();

    // private contribution
    let t = ScalarField::rand(&mut rng);

    let mut new_contributions = Contributions::default();

    for (idx, sub_contribution) in prev_contributions.sub_contributions.into_iter().enumerate() {
        let num_g1_powers = sub_contribution.powers_of_tau.g1_powers.len();
        let num_g2_powers = sub_contribution.powers_of_tau.g2_powers.len();

        // g1 powers
        let ptau_g1_contributed: Vec<G1> = sub_contribution
            .powers_of_tau
            .g1_powers
            .into_par_iter()
            .enumerate()
            .map(|(i, sg)| {
                G1Affine::from(sg)
                    .mul(t.pow([i as u64]))
                    .into_affine()
                    .into()
            })
            .collect::<Vec<_>>();

        // g2 powers
        let ptau_g2_contributed: Vec<G2> = sub_contribution
            .powers_of_tau
            .g2_powers
            .into_par_iter()
            .enumerate()
            .map(|(i, sg)| {
                G2Affine::from(sg)
                    .mul(t.pow([i as u64]))
                    .into_affine()
                    .into()
            })
            .collect::<Vec<_>>();

        let new_sub_contribution = Contribution::new(
            num_g1_powers,
            num_g2_powers,
            ptau_g1_contributed,
            ptau_g2_contributed,
            None,
        );

        new_contributions.sub_contributions[idx] = new_sub_contribution;
    }

    Ok(new_contributions)
}

#[wasm_bindgen]
pub fn xxx(n: usize) -> Promise {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_thread_pool(n)
}

#[wasm_bindgen]
pub fn contribute_wasm(input: &str) -> String {
    let response = contribute_with_string(input.to_string()).unwrap();
    return format!("{}", response);
}
