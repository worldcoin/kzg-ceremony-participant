mod contribution;

#[cfg(target_family = "wasm")]
mod wasm;

use std::time::Instant;
use std::{fs::File, path::Path};

use ark_bls12_381::{Fr as ScalarField, G1Affine, G2Affine};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Field;
use ark_serialize::Read;
use ark_serialize::Write;
use ark_std::UniformRand;
use blst::blst_p1_affine_in_g1;
use eyre::Result;
use rand::thread_rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

use crate::contribution::*;

const MAX_POWERS_OF_TAU: usize = 1 << 15;

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
    // let mut rng = thread_rng();
    let mut rng = ChaCha8Rng::seed_from_u64(1337);

    // private contribution
    let t = ScalarField::rand(&mut rng);

    // calculate powers of tau
    let ptau = (0..MAX_POWERS_OF_TAU)
        .into_par_iter()
        .map(|i| t.pow([i as u64]))
        .collect::<Vec<_>>();

    // we assume the last contribution entry contains all elements of the setup
    let contributions = prev_contributions.sub_contributions.to_vec();
    let full_contribution = contributions.last().unwrap();

    // calculate all the g1 powers
    // let start = Instant::now();
    let all_g1_tau: Vec<G1BlstProjective> = full_contribution
        .powers_of_tau
        .g1_powers
        .par_iter()
        .enumerate()
        .map(|(i, &sg)| {
            let p: G1BlstAffine = sg.into();
            let p: G1BlstProjective = p.into();
            p.mul(ptau[i].into())
        })
        .collect::<Vec<_>>()
        .into();
    // println!("add tau: {:?}", start.elapsed());

    let all_g1_tau: G1BlstProjectiveBatch = all_g1_tau.into();

    // let start = Instant::now();
    let all_g1_tau: G1BlstAffineBatch = all_g1_tau.into();
    // println!("projective to affine: {:?}", start.elapsed());

    let all_g1_tau: Vec<G1BlstAffine> = all_g1_tau.into();

    // let start = Instant::now();
    let all_g1_tau: Vec<G1> = all_g1_tau
        .into_par_iter()
        .map(|sg| {
            assert!(sg.is_in_subgroup());
            sg.into()
        })
        .collect::<Vec<_>>();
    // println!("subgroup check: {:?}", start.elapsed());

    // calculate the g2 powers (always same size)
    // let start = Instant::now();
    let all_g2_tau: Vec<G2> = full_contribution
        .powers_of_tau
        .g2_powers
        .par_iter()
        .enumerate()
        .map(|(i, &sg)| G2Affine::from(sg).mul(ptau[i]).into_affine().into())
        .collect::<Vec<_>>();
    // println!("g2 operations: {:?}", start.elapsed());

    // fill our data structure with the result
    let contributions = prev_contributions
        .sub_contributions
        .to_vec()
        .into_par_iter()
        .map(|sub_contribution| {
            let num_g1_powers = sub_contribution.powers_of_tau.g1_powers.len();
            let num_g2_powers = sub_contribution.powers_of_tau.g2_powers.len();

            Contribution::new(
                num_g1_powers,
                num_g2_powers,
                all_g1_tau[..num_g1_powers].to_vec(),
                all_g2_tau[..num_g2_powers].to_vec(),
                None,
            )
        })
        .collect::<Vec<_>>();

    let mut new_contributions = Contributions::default();
    for (idx, c) in contributions.into_iter().enumerate() {
        new_contributions.sub_contributions[idx] = c;
    }

    Ok(new_contributions)
}

// fn verify() {
//     // todo: pairing check
// }

#[cfg(test)]
pub mod test {
    // use std::mem::MaybeUninit;

    // use ark_bls12_381::{G1Affine, G2Affine};
    // use ark_ec::AffineCurve;
    // use blst::{blst_p1_generator, blst_p1_mult, blst_p1};
    // use ruint::{aliases::U384, uint};

    // use crate::contribution::{G1, G2, U768};

    // #[test]
    // fn test_blst() {
    //     let p = uint!(0x97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb_U384);
    //     let x : G1 = p.into();

    //     let y : G1BLST;
    //     unsafe {
    //         let mut tmp = std::mem::MaybeUninit::<blst_p1>::zeroed();
    //         tmp.write(*blst_p1_generator());
    //         y = tmp.into();
    //     }
    //     let y: G1 = y.into();

    //     assert_eq!(x, y);

    //     // other direction

    //     let xx : G1BLST = x.into();
    //     let xx : G1 = xx.into();

    //     assert_eq!(x, xx);
    // }

    // #[test]
    // fn test_serialize_g1() {
    //     let g1 = ark_bls12_381::G1Affine::prime_subgroup_generator();
    //     let p: G1 = g1.into();
    //     let p: U384 = p.into();
    //     let p = format!("{:#02x}", p);

    //     assert_eq!(p, "0x97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb");
    // }

    // #[test]
    // fn test_deserialize_g1() {
    //     let g1 = ark_bls12_381::G1Affine::prime_subgroup_generator();
    //     let p = uint!(0x97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb_U384);
    //     let p: G1 = p.into();
    //     let p: G1Affine = g1.into();

    //     assert_eq!(p, g1);
    // }

    // #[test]
    // fn test_serialize_g2() {
    //     let g2 = ark_bls12_381::G2Affine::prime_subgroup_generator();
    //     let p: G2 = g2.into();
    //     let p: U768 = p.into();
    //     let p = format!("{:#02x}", p);

    //     assert_eq!(p, "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8");
    // }

    // #[test]
    // fn test_deserialize_g2() {
    //     let g2 = ark_bls12_381::G2Affine::prime_subgroup_generator();
    //     let p = uint!(0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8_U768);
    //     let p: G2 = p.into();
    //     let p: G2Affine = g2.into();

    //     assert_eq!(p, g2);
    // }
}
