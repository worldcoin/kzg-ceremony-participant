mod contribution;
use std::{fs::File, path::Path, time::Instant};

use ark_bls12_381::{Fr as ScalarField, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Field;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, Write};
use ark_std::{cfg_into_iter, UniformRand};
use contribution::{PowersOfTau, U768};
use eyre::Result;
use rand::thread_rng;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::contribution::*;

/**
 * Serialize ptau to a file.
 */
fn write_ptau_file(ptau_g1: Vec<G1Affine>, ptau_g2: Vec<G2Affine>, path: &Path) -> Result<()> {
    let mut file = std::fs::File::create(path)?;
    for t in ptau_g1.into_iter() {
        t.serialize(&mut file)?;
    }
    for t in ptau_g2.into_iter() {
        t.serialize(&mut file)?;
    }
    Ok(())
}

/**
 * Apply a user's contibution to the setup
 */
fn contribute() -> Result<()> {
    println!("contribute");
    let mut rng = thread_rng();

    // private contribution
    let t = ScalarField::rand(&mut rng);

    let mut file = File::open("initialContribution.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let initial = serde_json::from_str(&data).unwrap();
    let initial = serde_json::from_value::<Contributions>(initial)?;

    let mut new_contributions: Vec<Contribution> = vec![];

    let start_total = Instant::now();

    for (idx, sub_contribution) in initial.sub_contributions.into_iter().enumerate() {
        let num_g1_powers = sub_contribution.powers_of_tau.g1_powers.len();
        let num_g2_powers = sub_contribution.powers_of_tau.g2_powers.len();

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

        let powers_of_tau = PowersOfTau {
            g1_powers: ptau_g1_contributed,
            g2_powers: ptau_g2_contributed,
        };

        let new_sub_contribution = Contribution {
            num_g1_powers,
            num_g2_powers,
            powers_of_tau,
            pot_pubkey: None,
        };

        new_contributions.push(new_sub_contribution);
    }

    println!("Total Duration: {:?}", start_total.elapsed());

    Ok(())
}

fn verify() {
    // todo: pairing check
}

fn main() {
    // generate_initial().unwrap();
    contribute().unwrap();
}
