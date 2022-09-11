use core::slice;
use std::mem::MaybeUninit;

use ark_bls12_381::{G1Affine, G2Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use blst::{
    blst_p1, blst_p1_affine, blst_p1_affine_compress, blst_p1_from_affine, blst_p1_mult,
    blst_p1_uncompress, blst_p1s_to_affine, blst_p1_to_affine, blst_p1_affine_in_g1,
};
use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use ruint::{
    aliases::{U256, U384},
    Uint,
};
use serde::{Deserialize, Serialize};

pub type U768 = Uint<768, 12>;

pub trait BlstAlgebra {
    fn mul(&self, scalar: U256) -> Self;
    fn is_in_subgroup(&self) -> bool;
}

pub struct G1BlstProjective(MaybeUninit<blst_p1>);
pub struct G1BlstAffine(MaybeUninit<blst_p1_affine>);
pub struct G1BlstProjectiveBatch(Vec<G1BlstProjective>);
pub struct G1BlstAffineBatch(Vec<G1BlstAffine>);

impl From<MaybeUninit<blst_p1_affine>> for G1BlstAffine {
    fn from(u: MaybeUninit<blst_p1_affine>) -> Self {
        G1BlstAffine(u)
    }
}

impl From<blst_p1_affine> for G1BlstAffine {
    fn from(u: blst_p1_affine) -> Self {
        let mut p = std::mem::MaybeUninit::<blst_p1_affine>::zeroed();
        p.write(u);
        p.into()
    }
}

impl From<G1BlstAffine> for MaybeUninit<blst_p1_affine> {
    fn from(u: G1BlstAffine) -> Self {
        u.0
    }
}

impl From<MaybeUninit<blst_p1>> for G1BlstProjective {
    fn from(u: MaybeUninit<blst_p1>) -> Self {
        G1BlstProjective(u)
    }
}

impl From<G1BlstProjective> for MaybeUninit<blst_p1> {
    fn from(u: G1BlstProjective) -> Self {
        u.0
    }
}

impl From<Vec<G1BlstProjective>> for G1BlstProjectiveBatch {
    fn from(u: Vec<G1BlstProjective>) -> Self {
        G1BlstProjectiveBatch(u)
    }
}

impl From<G1BlstProjectiveBatch> for Vec<G1BlstProjective> {
    fn from(u: G1BlstProjectiveBatch) -> Self {
        u.0
    }
}

impl From<Vec<G1BlstAffine>> for G1BlstAffineBatch {
    fn from(u: Vec<G1BlstAffine>) -> Self {
        G1BlstAffineBatch(u)
    }
}

impl From<G1BlstAffineBatch> for Vec<G1BlstAffine> {
    fn from(u: G1BlstAffineBatch) -> Self {
        u.0
    }
}

impl From<G1BlstProjectiveBatch> for G1BlstAffineBatch {
    fn from(u: G1BlstProjectiveBatch) -> Self {
        let size = u.0.len();
        let input = u.0.iter().map(|x| (*x).0.as_ptr()).collect::<Vec<_>>();
        let mut out = Vec::<blst_p1_affine>::with_capacity(size);

        unsafe {
            blst_p1s_to_affine( out.as_mut_ptr(), input.as_ptr(), size);
            out.set_len(size);

            G1BlstAffineBatch(out.into_par_iter().map(|x| x.into()).collect::<Vec<G1BlstAffine>>())
        }
    }
}

impl From<G1BlstProjective> for G1BlstAffine {
    fn from(u: G1BlstProjective) -> Self {
        let mut p = std::mem::MaybeUninit::<blst_p1_affine>::zeroed();
        unsafe {
            blst_p1_to_affine(p.as_mut_ptr(), u.0.as_ptr());
        }
        p.into()
    }
}

impl From<G1BlstAffine> for G1BlstProjective {
    fn from(u: G1BlstAffine) -> Self {
        let mut p = std::mem::MaybeUninit::<blst_p1>::zeroed();
        unsafe {
            blst_p1_from_affine(p.as_mut_ptr(), u.0.as_ptr());
        }
        p.into()
    }
}

impl BlstAlgebra for G1BlstProjective {
    fn mul(&self, scalar: U256) -> Self {
        let mut buffer: [u8; 32] = scalar.as_le_slice().try_into().unwrap();

        let mut tmp = MaybeUninit::<blst_p1>::zeroed();
        unsafe {
            blst_p1_mult(tmp.as_mut_ptr(), self.0.as_ptr(), buffer.as_ptr(), 256);
        }
        tmp.into()
    }

    fn is_in_subgroup(&self) -> bool {
        // TODO
        false
    }
}

impl BlstAlgebra for G1BlstAffine {
    fn mul(&self, scalar: U256) -> Self {
        // TODO
        Self(MaybeUninit::zeroed())
    }

    fn is_in_subgroup(&self) -> bool {
        unsafe {
            blst_p1_affine_in_g1(self.0.as_ptr())
        }
    }
}
    
impl From<G1BlstAffine> for G1 {
    fn from(g: G1BlstAffine) -> Self {
        let mut buffer = [0u8; 48];
        unsafe {
            blst_p1_affine_compress(buffer.as_mut_ptr(), g.0.as_ptr());
        }
        buffer.reverse();

        G1(U384::from_le_bytes(buffer))
    }
}

impl From<G1> for G1BlstAffine {
    fn from(g: G1) -> Self {
        let mut buffer: [u8; 48] = g.0.as_le_slice().try_into().unwrap();
        buffer.reverse();

        let mut p = std::mem::MaybeUninit::<blst_p1_affine>::zeroed();
        unsafe {
            blst_p1_uncompress(p.as_mut_ptr(), buffer.as_ptr());
        }

        p.into()
    }
}

///////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum IdType {
    EthAddress,
    EnsName,
    GithubHandle,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributeStartRequest {
    id_type: IdType,
    id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct G1(U384);

impl From<U384> for G1 {
    fn from(u: U384) -> Self {
        G1(u)
    }
}

impl From<G1> for U384 {
    fn from(u: G1) -> Self {
        u.0
    }
}

impl From<G1Affine> for G1 {
    fn from(g: G1Affine) -> Self {
        let mut buffer = [0u8; 48];

        g.serialize(&mut buffer[..])
            .expect("g1 serialization failed");
        // set the third most significant bit to the same as the first bit (signal)
        buffer[47] &= ((buffer[47] & 0x20) << 2) | 0x7F;
        // set the most significant bit to 1 (compressed form)
        buffer[47] |= 0x80;
        G1(U384::from_le_bytes(buffer))
    }
}

impl From<G1> for G1Affine {
    fn from(g: G1) -> Self {
        let mut buffer: [u8; 48] = g.0.as_le_slice().try_into().unwrap();
        // set the most significant bit to the same as the third bit (signal)
        buffer[47] &= ((buffer[47] & 0x80) >> 2) | 0xDF;
        G1Affine::deserialize(&mut &buffer[..]).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct G2(U768);

impl From<U768> for G2 {
    fn from(u: U768) -> Self {
        G2(u)
    }
}

impl From<G2> for U768 {
    fn from(u: G2) -> Self {
        u.0
    }
}

impl From<G2Affine> for G2 {
    fn from(g: G2Affine) -> Self {
        let mut buffer = [0u8; 96];
        g.serialize(&mut buffer[..])
            .expect("g2 serialization failed");
        // set the third most significant bit to the same as the first bit (signal)
        buffer[95] &= ((buffer[95] & 0x20) << 2) | 0x7F;
        // set the most significant bit to 1 (compressed form)
        buffer[95] |= 0x80;
        G2(U768::from_le_bytes(buffer))
    }
}

impl From<G2> for G2Affine {
    fn from(g: G2) -> Self {
        let mut buffer: [u8; 96] = g.0.as_le_slice().try_into().unwrap();
        // set the most significant bit to the same as the third bit (signal)
        buffer[95] &= ((buffer[95] & 0x80) >> 2) | 0xDF;
        G2Affine::deserialize(&mut &buffer[..]).unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contribution {
    pub num_g1_powers: usize,
    pub num_g2_powers: usize,
    pub powers_of_tau: PowersOfTau,
    pub pot_pubkey: Option<G2>,
}

impl Contribution {
    pub fn new(
        num_g1_powers: usize,
        num_g2_powers: usize,
        g1_powers: Vec<G1>,
        g2_powers: Vec<G2>,
        pot_pubkey: Option<G2>,
    ) -> Self {
        Self {
            num_g1_powers,
            num_g2_powers,
            powers_of_tau: PowersOfTau::new(g1_powers, g2_powers),
            pot_pubkey,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowersOfTau {
    pub g1_powers: Vec<G1>,
    pub g2_powers: Vec<G2>,
}

impl PowersOfTau {
    pub fn new(g1_powers: Vec<G1>, g2_powers: Vec<G2>) -> Self {
        Self {
            g1_powers,
            g2_powers,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributions {
    pub sub_contributions: [Contribution; 4],
}

impl Default for Contributions {
    fn default() -> Self {
        Self {
            sub_contributions: [
                Contribution::new(0, 0, vec![], vec![], None),
                Contribution::new(0, 0, vec![], vec![], None),
                Contribution::new(0, 0, vec![], vec![], None),
                Contribution::new(0, 0, vec![], vec![], None),
            ],
        }
    }
}
