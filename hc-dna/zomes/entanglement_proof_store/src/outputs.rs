use hdk::prelude::*;
use holo_hash::DnaHash;

use crate::{EntanglementProofExpression};

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct ExpressionResponse {
    //#[serde(flatten)]
    pub expression_data: EntanglementProofExpression,
    pub holochain_data: HolochainData,
}

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct HolochainData {
    pub element: Element,
    pub expression_dna: DnaHash,
    pub creator: AgentPubKey,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct ManyExpressionResponse(pub Vec<ExpressionResponse>);

#[derive(SerializedBytes, Serialize, Deserialize, Debug)]
pub struct MaybeExpression(pub Option<ExpressionResponse>);

#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct ManyDhtHash(pub Vec<DnaHash>);
