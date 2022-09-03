mod contribution;

// feature flag
#[cfg(target_arch = "wasm32-unknown-unknown")]
mod wasm;

use std::panic;
use std::{fs::File, path::Path, time::Instant};

use ark_bls12_381::{Fr as ScalarField, G1Affine, G2Affine};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Field;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use ark_serialize::Read;
use ark_serialize::Write;
use ark_std::UniformRand;
use eyre::Result;
use rand::thread_rng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use ruint::aliases::U384;
use ruint::uint;
use ruint::Uint;

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

    let start_total = Instant::now();

    let contributions = prev_contributions
        .sub_contributions
        .to_vec()
        .into_par_iter()
        .map(|sub_contribution| {
            let num_g1_powers = sub_contribution.powers_of_tau.g1_powers.len();
            let num_g2_powers = sub_contribution.powers_of_tau.g2_powers.len();

            // g1 powers
            let start = Instant::now();
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
            let duration = start.elapsed();
            println!("g1 Duration: {:?}", duration);

            // g2 powers
            let start = Instant::now();
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
            let duration = start.elapsed();
            println!("g2 Duration: {:?}", duration);

            Contribution::new(
                num_g1_powers,
                num_g2_powers,
                ptau_g1_contributed,
                ptau_g2_contributed,
                None,
            )
        })
        .collect::<Vec<_>>();

    let mut new_contributions = Contributions::default();
    for (idx, c) in contributions.into_iter().enumerate() {
        new_contributions.sub_contributions[idx] = c;
    }

    println!("Total Duration: {:?}", start_total.elapsed());

    Ok(new_contributions)
}

// fn verify() {
//     // todo: pairing check
// }

#[cfg(test)]
pub mod test {
    use ark_bls12_381::G1Affine;
    use ark_ec::AffineCurve;
    use ruint::{aliases::U384, uint};

    use crate::contribution::G1;

    #[test]
    fn test_serialize() {
        let g1 = ark_bls12_381::G1Affine::prime_subgroup_generator();
        let p: G1 = g1.into();
        let p: U384 = p.into();
        let p = format!("{:#02x}", p);

        assert_eq!(p, "0x97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb");
    }

    #[test]
    fn test_deserialize() {
        let g1 = ark_bls12_381::G1Affine::prime_subgroup_generator();
        let p = uint!(0x97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb_U384);
        let p: G1 = p.into();
        let p: G1Affine = g1.into();

        assert_eq!(p, g1);
    }
}