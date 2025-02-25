use accounting::state::transfer;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cw_storage_plus::Map;
use thiserror::Error;
use tinyvec::TinyVec;
use wormhole::vaa::Signature;

use crate::msg::Observation;

pub const PENDING_TRANSFERS: Map<transfer::Key, TinyVec<[Data; 2]>> = Map::new("pending_transfers");
pub const CHAIN_REGISTRATIONS: Map<u16, Binary> = Map::new("chain_registrations");
pub const DIGESTS: Map<(u16, Vec<u8>, u64), Binary> = Map::new("digests");

#[cw_serde]
pub struct PendingTransfer {
    pub key: transfer::Key,
    pub data: Vec<Data>,
}

#[derive(Error, Debug)]
#[error("cannot submit duplicate signatures for the same observation")]
pub struct DuplicateSignatureError;

#[cw_serde]
#[derive(Default)]
pub struct Data {
    observation: Observation,

    guardian_set_index: u32,

    signatures: Vec<Signature>,
}

impl Data {
    pub const fn new(observation: Observation, guardian_set_index: u32) -> Self {
        Self {
            observation,
            guardian_set_index,
            signatures: Vec::new(),
        }
    }

    pub fn observation(&self) -> &Observation {
        &self.observation
    }

    pub fn guardian_set_index(&self) -> u32 {
        self.guardian_set_index
    }

    pub fn signatures(&self) -> &[Signature] {
        &self.signatures
    }

    /// Returns true if there is a signature associated with `index` in this `Data`.
    pub fn has_signature(&self, index: u8) -> bool {
        self.signatures
            .binary_search_by_key(&index, |s| s.index)
            .is_ok()
    }

    /// Adds `sig` to the list of signatures for this transfer data.  Returns true if `sig`
    /// was successfully added or false if `sig` was already in the signature list.
    pub fn add_signature(&mut self, sig: Signature) -> Result<(), DuplicateSignatureError> {
        match self
            .signatures
            .binary_search_by_key(&sig.index, |s| s.index)
        {
            Ok(_) => Err(DuplicateSignatureError),
            Err(idx) => {
                self.signatures.insert(idx, sig);
                Ok(())
            }
        }
    }
}
