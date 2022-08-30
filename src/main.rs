use std::path::Path;

use ark_bls12_381::{Fr as ScalarField, G1Affine as GAffine, G1Projective as G};
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Field;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Write};
use ark_std::{cfg_into_iter, UniformRand};
use eyre::Result;
use rand::thread_rng;

const DOMAIN_SIZE: u32 = 12; // support up to 2**12=4096 field elements

fn write_file(ptau: Vec<GAffine>, path: &Path) -> Result<()> {
    let mut compressed_bytes = Vec::new();
    for t in ptau.into_iter() {
        t.serialize(&mut compressed_bytes)?;
    }

    let mut file = std::fs::File::create(path)?;
    file.write_all(&compressed_bytes)?;
    Ok(())
}

// use this function to generate the initial ptau setup
fn generate_initial() -> Result<()> {
    let mut rng = thread_rng();

    // g1 generator
    let g1 = G::rand(&mut rng).into_affine();

    // secret
    let s = ScalarField::rand(&mut rng);

    let ptau = cfg_into_iter!(0..u32::pow(2, DOMAIN_SIZE) + 1)
        .map(|i| g1.mul(s.pow([i as u64])).into_affine())
        .collect::<Vec<_>>();

    write_file(ptau, Path::new("setup.ptau"))
}

// use this function to apply a user's contibution to the setup
fn contribute() -> Result<()> {
    let mut rng = thread_rng();
    let mut ptau: Vec<GAffine> = Vec::new();

    // read ptau file
    let mut reader = std::fs::File::open("setup.ptau")?;
    for _ in 0..u32::pow(2, DOMAIN_SIZE) + 1 {
        ptau.push(GAffine::deserialize(&mut reader)?);
    }

    // private contribution
    let t = ScalarField::rand(&mut rng);

    // apply to existing ptau
    let ptau_contributed = cfg_into_iter!(ptau).enumerate()
        .map(|(i, sg)| sg.mul(t.pow([i as u64])).into_affine())
        .collect::<Vec<_>>();

    write_file(ptau_contributed, Path::new("contribution.ptau"))
}

fn verify() {
    // todo: pairing check
}

fn main() {
    generate_initial().unwrap();
}
