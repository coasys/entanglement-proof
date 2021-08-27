use hc_time_index::IndexableEntry;
use hdk::prelude::*;
use chrono::{Utc, DateTime};

use crate::EntanglementProofExpression;

impl IndexableEntry for EntanglementProofExpression {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}