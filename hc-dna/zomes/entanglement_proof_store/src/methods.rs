use hdk::prelude::*;
use chrono::{DateTime, Utc};

use crate::{inputs::CreateExpression, outputs::HolochainData, utils::err};
use crate::{
    ExpressionDNA, ExpressionResponse, EntanglementProofExpression,
};
use crate::errors::ExpressionResult;

impl ExpressionDNA {
    /// Create an expression and link it to yourself publicly
    pub fn create_public_expression(content: CreateExpression) -> ExpressionResult<ExpressionResponse> {
        // Serialize data to check its valid and prepare for entry into source chain
        let expression = EntanglementProofExpression::try_from(content)?;
        let expression_hash = hash_entry(&expression)?;
        create_entry(&expression)?;

        //Create time index for did author so that get_by_author can query with time pagination
        hc_time_index::index_entry(expression.author.clone(), expression.clone(), LinkTag::new("expression"))?;

        let expression_element = get(expression_hash, GetOptions::default())?
            .ok_or(err("Could not get entry after commit"))?;
        let timestamp = expression_element.header().timestamp();

        Ok(ExpressionResponse {
            expression_data: expression,
            holochain_data: HolochainData {
                element: expression_element,
                expression_dna: zome_info()?.dna_hash,
                creator: agent_info()?.agent_latest_pubkey,
                created_at: chrono::DateTime::from_utc(
                    chrono::naive::NaiveDateTime::from_timestamp(timestamp.0, timestamp.1),
                    chrono::Utc,
                ),
            },
        })
    }

    /// Get expressions authored by a given Agent/Identity
    pub fn get_by_author(
        author: String,
        from: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> ExpressionResult<Vec<ExpressionResponse>> {
        let links = hc_time_index::get_links_for_time_span(author, from, until, Some(LinkTag::new("expression")), None)?;
        Ok(links
            .into_iter()
            .map(|link| {
                let expression_element = get(link.target, GetOptions::default())?
                    .ok_or(err("Could not get entry after commit"))?;
                let timestamp = expression_element.header().timestamp();
                let exp_data: EntanglementProofExpression = expression_element
                    .entry()
                    .to_app_option()?
                    .ok_or(WasmError::Host(String::from(
                        "Could not deserialize link expression data into EntanglementProofExpression",
                    )))?;
                Ok(ExpressionResponse {
                    expression_data: exp_data,
                    holochain_data: HolochainData {
                        element: expression_element,
                        expression_dna: zome_info()?.dna_hash,
                        creator: agent_info()?.agent_latest_pubkey,
                        created_at: chrono::DateTime::from_utc(
                            chrono::naive::NaiveDateTime::from_timestamp(timestamp.0, timestamp.1),
                            chrono::Utc,
                        ),
                    },
                })
            })
            .collect::<Result<Vec<ExpressionResponse>, WasmError>>()?)
    }

    pub fn get_expression_by_address(
        address: AnyDhtHash,
    ) -> ExpressionResult<Option<ExpressionResponse>> {
        let expression = get(address, GetOptions::default())?;
        match expression {
            Some(expression_element) => {
                let exp_data: EntanglementProofExpression = expression_element
                    .entry()
                    .to_app_option()?
                    .ok_or(WasmError::Host(String::from(
                        "Could not deserialize link expression data into EntanglementProofExpression",
                    )))?;
                let timestamp = expression_element.header().timestamp();
                Ok(Some(ExpressionResponse {
                    expression_data: exp_data,
                    holochain_data: HolochainData {
                        element: expression_element,
                        expression_dna: zome_info()?.dna_hash,
                        creator: agent_info()?.agent_latest_pubkey,
                        created_at: chrono::DateTime::from_utc(
                            chrono::naive::NaiveDateTime::from_timestamp(timestamp.0, timestamp.1),
                            chrono::Utc,
                        ),
                    },
                }))
            }
            None => Ok(None),
        }
    }
}
