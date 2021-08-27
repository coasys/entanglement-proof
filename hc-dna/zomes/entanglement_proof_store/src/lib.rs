use chrono::{DateTime, Utc};
use hdk::prelude::*;

mod inputs;
mod methods;
mod outputs;
mod utils;
mod impls;
mod errors;

use inputs::*;
use outputs::*;

/// Expression data this DNA is "hosting"
#[hdk_entry(id = "shortform_expression", visibility = "public")]
#[derive(Clone)]
pub struct EntanglementProofExpression {
    data: EntanglementProofData,
    author: String,
    timestamp: DateTime<Utc>,
    proof: ExpressionProof,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct EntanglementProofData {
    did: String,
    #[serde(rename(serialize = "deviceKey"))]
    #[serde(rename(deserialize = "deviceKey"))]
    device_key: String,

    #[serde(rename(serialize = "deviceKeySignedByDid"))]
    #[serde(rename(deserialize = "deviceKeySignedByDid"))]
    device_key_signed_by_did: String,

    #[serde(rename(serialize = "didSignedByDeviceKey"))]
    #[serde(rename(deserialize = "didSignedByDeviceKey"))]
    did_signed_by_device_key: String,
}

pub struct ExpressionDNA();

entry_defs![
    EntanglementProofExpression::entry_def(),
    Path::entry_def()
];

// Zome functions

/// Run function where zome is init'd by agent. This adds open cap grant for recv_private_expression function
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

/// Create an expression and link it to yourself publicly
#[hdk_extern]
pub fn create_public_expression(create_data: CreateExpression) -> ExternResult<ExpressionResponse> {
    Ok(ExpressionDNA::create_public_expression(create_data).map_err(|err| WasmError::Host(err.to_string()))?)
}

/// Get expressions authored by a given Agent/Identity
#[hdk_extern]
pub fn get_by_author(get_data: GetByAuthor) -> ExternResult<ManyExpressionResponse> {
    Ok(ManyExpressionResponse(ExpressionDNA::get_by_author(
        get_data.author,
        get_data.from,
        get_data.until,
    ).map_err(|err| WasmError::Host(err.to_string()))?))
}

#[hdk_extern]
pub fn get_expression_by_address(address: AnyDhtHash) -> ExternResult<MaybeExpression> {
    Ok(MaybeExpression(ExpressionDNA::get_expression_by_address(
        address,
    ).map_err(|err| WasmError::Host(err.to_string()))?))
}


// Validation functions
