use chrono::{DateTime, Utc};
use hdk::prelude::*;

use crate::{EntanglementProofExpression, EntanglementProofData};

use crate::utils::err;

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct CreateExpression {
    pub data: String,
    pub author: String,
    pub timestamp: DateTime<Utc>,
    pub proof: ExpressionProof,
}

#[derive(SerializedBytes, Serialize, Deserialize, Clone, Debug)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

impl TryFrom<CreateExpression> for EntanglementProofExpression {
    type Error = WasmError;

    fn try_from(content: CreateExpression) -> Result<Self, Self::Error> {
        let expression: EntanglementProofData = serde_json::from_str(&content.data)
            .map_err(|_| err("Could not serialized content into ShortForm expression type"))?;

        Ok(EntanglementProofExpression {
            data: expression,
            author: content.author,
            timestamp: content.timestamp,
            proof: content.proof,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetByAuthor {
    pub author: String,
    pub from: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct SendPrivate {
    pub to: AgentPubKey,
    pub expression: CreateExpression,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct Inbox {
    pub from: Option<String>,
    pub page_size: usize,
    pub page_number: usize,
}
