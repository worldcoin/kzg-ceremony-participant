use std::path::Path;

use ark_bls12_381::{Fr as ScalarField, G1Affine as G1Affine, G1Projective as G1, G2Affine as G2Affine, G2Projective as G2};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Field;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Write};
use ark_std::{cfg_into_iter, UniformRand};
use eyre::Result;
use rand::thread_rng;

const DOMAIN_SIZE: u32 = 12; // support up to 2**12=4096 field elements

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
 * Generate the initial ptau setup.
 */
fn generate_initial() -> Result<()> {
    let mut rng = thread_rng();

    // g1 generator
    let g1 = G1::rand(&mut rng).into_affine();
    let g2 = G2::rand(&mut rng).into_affine();

    // secret
    let s = ScalarField::rand(&mut rng);

    // s^i*G1
    let ptau_g1 = cfg_into_iter!(0..u32::pow(2, DOMAIN_SIZE) + 1)
        .map(|i| g1.mul(s.pow([i as u64])).into_affine())
        .collect::<Vec<_>>();

    // s^i*G2
    let ptau_g2 = cfg_into_iter!(0..u32::pow(2, DOMAIN_SIZE) + 1)
        .map(|i| g2.mul(s.pow([i as u64])).into_affine())
        .collect::<Vec<_>>();

    // serialize and write to file
    write_ptau_file(ptau_g1, ptau_g2, Path::new("setup.ptau"))
}

/**
 * Apply a user's contibution to the setup
 */
fn contribute() -> Result<()> {
    let mut rng = thread_rng();
    
    // read ptau file
    let mut ptau_g1: Vec<G1Affine> = Vec::new();
    let mut ptau_g2: Vec<G2Affine> = Vec::new();

    let mut reader = std::fs::File::open("setup.ptau")?;
    for _ in 0..u32::pow(2, DOMAIN_SIZE) + 1 {
        ptau_g1.push(G1Affine::deserialize(&mut reader)?);
    }

    for _ in 0..u32::pow(2, DOMAIN_SIZE) + 1 {
        ptau_g2.push(G2Affine::deserialize(&mut reader)?);
    }

    // private contribution
    let t = ScalarField::rand(&mut rng);

    // apply to existing ptau
    let ptau_g1_contributed = cfg_into_iter!(ptau_g1).enumerate()
        .map(|(i, sg)| sg.mul(t.pow([i as u64])).into_affine())
        .collect::<Vec<_>>();

    let ptau_g2_contributed = cfg_into_iter!(ptau_g2).enumerate()
        .map(|(i, sg)| sg.mul(t.pow([i as u64])).into_affine())
        .collect::<Vec<_>>();

    // serialize and write to file
    write_ptau_file(ptau_g1_contributed, ptau_g2_contributed, Path::new("contribution.ptau"))
}

fn verify() {
    // todo: pairing check
}

fn main() {
    generate_initial().unwrap();
    contribute().unwrap();
}
