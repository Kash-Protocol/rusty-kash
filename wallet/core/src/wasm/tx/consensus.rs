use crate::tx::consensus as core;
use kash_addresses::Address;
use kash_consensus_core::{config::params::Params, network::NetworkType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ConsensusParams {
    params: Params,
}

impl From<Params> for ConsensusParams {
    fn from(params: Params) -> Self {
        Self { params }
    }
}

impl From<ConsensusParams> for Params {
    fn from(cp: ConsensusParams) -> Self {
        cp.params
    }
}

/// find Consensus parameters for given Address
#[wasm_bindgen(js_name = getConsensusParametersByAddress)]
pub fn get_consensus_params_by_address(address: &Address) -> ConsensusParams {
    core::get_consensus_params_by_address(address).into()
}

/// find Consensus parameters for given NetworkType
#[wasm_bindgen(js_name = getConsensusParametersByNetwork)]
pub fn get_consensus_params_by_network(network: NetworkType) -> ConsensusParams {
    core::get_consensus_params_by_network(&network).into()
}
