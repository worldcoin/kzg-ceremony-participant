use std::mem::MaybeUninit;

use ark_bls12_381::{G1Affine, G2Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use blst::{blst_encode_to_g1, blst_p1, blst_p1_compress, blst_p1_uncompress, blst_p1_affine, blst_p1_to_affine, blst_p1_affine_compress, blst_p1_from_affine, blst_p1_mult, blst_scalar, blst_scalar_from_le_bytes};
use ruint::{aliases::{U384, U256}, Uint};
use serde::{Deserialize, Serialize};

pub type U768 = Uint<768, 12>;

pub trait BLST {
    fn mul(&self, scalar: U256) -> Self;
}
pub struct G1BLST(MaybeUninit<blst_p1>);

impl From<MaybeUninit<blst_p1>> for G1BLST {
    fn from(u: MaybeUninit<blst_p1>) -> Self {
        G1BLST(u)
    }
}

impl From<G1BLST> for MaybeUninit<blst_p1> {
    fn from(u: G1BLST) -> Self {
        u.0
    }
}

impl BLST for G1BLST {
    fn mul(&self, scalar: U256) -> Self {
        let mut buffer: [u8; 32] = scalar.as_le_slice().try_into().unwrap();

        let mut tmp = MaybeUninit::<blst_p1>::zeroed();
        unsafe {
            blst_p1_mult(tmp.as_mut_ptr(), self.0.as_ptr(), buffer.as_ptr(), 256);
        }
        tmp.into()
    }
}

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

impl From<G1BLST> for G1 {
    fn from(g: G1BLST) -> Self {
        let mut buffer = [0u8; 48];
        unsafe {
            blst_p1_compress(buffer.as_mut_ptr(), g.0.as_ptr());
        }
        buffer.reverse();
        G1(U384::from_le_bytes(buffer))
    }
}

impl From<G1> for G1BLST {
    fn from(g: G1) -> Self {
        let mut buffer: [u8; 48] = g.0.as_le_slice().try_into().unwrap();
        buffer.reverse();

        let mut p1 = std::mem::MaybeUninit::<blst_p1_affine>::zeroed();
        unsafe {
            blst_p1_uncompress(p1.as_mut_ptr(), buffer.as_ptr());
        }

        let mut p2 = std::mem::MaybeUninit::<blst_p1>::zeroed();
        unsafe {
            blst_p1_from_affine(p2.as_mut_ptr(), p1.as_ptr())
        }

        p2.into()
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
