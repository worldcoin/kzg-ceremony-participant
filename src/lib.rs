#[cfg(target_family = "wasm")]
mod wasm;

use std::{fs::File, path::Path};

use ark_bls12_381::{Fr as ScalarField, G1Affine, G2Affine};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Field;
use ark_serialize::Read;
use ark_serialize::Write;
use ark_std::UniformRand;
use eyre::Result;
use kzg_ceremony_crypto::ContributionJson;
use kzg_ceremony_crypto::ContributionsJson;
use rand::thread_rng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::IntoParallelRefMutIterator;

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
    let post = contribute(ContributionsJson::from_json(&json)?.parse()?)?;
    Ok(serde_json::to_string(&post)?)
}

/**
 * Apply a user's contibution to the setup
 */
fn contribute(
    contributions: Vec<kzg_ceremony_crypto::Contribution>,
) -> Result<kzg_ceremony_crypto::ContributionsJson> {
    let mut post_contributions = ContributionsJson::initial();

    // generate randomness
    // TODO: add externally generated seed based on user input
    let mut rng = thread_rng();

    // private contribution
    let tau = ScalarField::rand(&mut rng);

    // we'll only use the last contribution, since all others are just subsets
    let mut full_contribution = contributions.last().unwrap().clone();

    //subgroup check
    full_contribution.subgroup_check();

    // actual contribution
    full_contribution.add_tau(&tau);

    // encode the contribution
    let contribution: ContributionJson = full_contribution.into();

    // construct the response
    post_contributions
        .sub_contributions
        .par_iter_mut()
        .map(|el| {
            el.powers_of_tau.g1_powers = contribution.powers_of_tau.g1_powers[..el.num_g1_powers].to_vec();
            el.powers_of_tau.g2_powers = contribution.powers_of_tau.g1_powers[..el.num_g2_powers].to_vec();
            // TODO: set potkey
        })
        .collect::<Vec<_>>();

    Ok(post_contributions)
}

#[cfg(test)]
pub mod test {
    use ark_bls12_381::{G1Affine, G2Affine};
    use ark_ec::AffineCurve;
    use ruint::{aliases::U384, uint};

    use crate::contribution::{G1, G2, U768};

    #[test]
    fn test_serialize_g1() {
        let g1 = ark_bls12_381::G1Affine::prime_subgroup_generator();
        let p: G1 = g1.into();
        let p: U384 = p.into();
        let p = format!("{:#02x}", p);

        assert_eq!(p, "0x97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb");
    }

    #[test]
    fn test_deserialize_g1() {
        let g1 = ark_bls12_381::G1Affine::prime_subgroup_generator();
        let p = uint!(0x97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb_U384);
        let p: G1 = p.into();
        let p: G1Affine = g1.into();

        assert_eq!(p, g1);
    }

    #[test]
    fn test_serialize_g2() {
        let g2 = ark_bls12_381::G2Affine::prime_subgroup_generator();
        let p: G2 = g2.into();
        let p: U768 = p.into();
        let p = format!("{:#02x}", p);

        assert_eq!(p, "0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8");
    }

    #[test]
    fn test_deserialize_g2() {
        let g2 = ark_bls12_381::G2Affine::prime_subgroup_generator();
        let p = uint!(0x93e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8_U768);
        let p: G2 = p.into();
        let p: G2Affine = g2.into();

        assert_eq!(p, g2);
    }
}
