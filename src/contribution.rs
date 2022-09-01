use ark_bls12_381::{G1Affine, G2Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ruint::{aliases::U384, Uint};
use serde::{Deserialize, Serialize};
use tracing::instrument;

pub type U768 = Uint<768, 12>;

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

impl From<G1Affine> for G1 {
    fn from(g: G1Affine) -> Self {
        let mut buffer = [0u8; 48];
        g.serialize(&mut buffer[..])
            .expect("g1 serialization failed");
        G1(U384::from_le_bytes(buffer))
    }
}

impl From<G1> for G1Affine {
    fn from(g: G1) -> Self {
        let buffer = g.0.as_le_slice();
        G1Affine::deserialize(buffer).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct G2(U768);

impl From<U768> for G2 {
    fn from(u: U768) -> Self {
        G2(u)
    }
}

impl From<G2Affine> for G2 {
    fn from(g: G2Affine) -> Self {
        let mut buffer = [0u8; 96];
        g.serialize(&mut buffer[..])
            .expect("g2 serialization failed");
        G2(U768::from_le_bytes(buffer))
    }
}

impl From<G2> for G2Affine {
    fn from(g: G2) -> Self {
        let buffer = g.0.as_le_slice();
        G2Affine::deserialize(buffer).unwrap()
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
