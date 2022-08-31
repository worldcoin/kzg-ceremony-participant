use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use axum::{extract::Json, routing::post, Router};
use ruint::{aliases::U384, Uint};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use valico::json_schema;
use ark_bls12_381::{G1Affine, G2Affine};

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
    id:      String,
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
        g.serialize(&mut buffer[..]);
        G1(U384::from_le_bytes(buffer))
    }
}

impl From<G1> for G1Affine {
    fn from(g: G1) -> Self {
        let mut buffer = g.0.as_le_slice();
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
        g.serialize(&mut buffer[..]);
        G2(U768::from_le_bytes(buffer))
    }
}

impl From<G2> for G2Affine {
    fn from(g: G2) -> Self {
        let mut buffer = g.0.as_le_slice();
        G2Affine::deserialize(buffer).unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contribution {
    pub num_g1_powers: usize,
    pub num_g2_powers: usize,
    pub powers_of_tau: PowersOfTau,
    pub pot_pubkey:    Option<G2>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowersOfTau {
    pub g1_powers: Vec<G1>,
    pub g2_powers: Vec<G2>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributions {
    pub sub_contributions: [Contribution; 4],
}

#[instrument]
pub async fn start(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    dbg!(&payload);

    Json(payload)
}