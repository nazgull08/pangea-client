use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use ethers_core::types::{Address, H256};

use crate::{
    core::types::{default_chains, ChainId},
    query::Bound,
    utils::serialize_comma_separated,
};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetMiraPoolsRequest {
    #[serde(default = "default_chains")]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(alias = "asset0__in", serialize_with = "serialize_comma_separated")]
    pub asset0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(alias = "asset1__in", serialize_with = "serialize_comma_separated")]
    pub asset1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub assets__in: HashSet<Address>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetMiraLiquidityRequest {
    #[serde(default = "default_chains")]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(alias = "asset0__in", serialize_with = "serialize_comma_separated")]
    pub asset0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(alias = "asset1__in", serialize_with = "serialize_comma_separated")]
    pub asset1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub assets__in: HashSet<Address>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct GetMiraSwapsRequest {
    #[serde(default = "default_chains")]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub chains: HashSet<ChainId>,

    // Inclusive lower bound if is Some for block number
    #[serde(default)]
    pub from_block: Bound,
    // Inclusive upper bound if is Some for block number
    #[serde(default)]
    pub to_block: Bound,

    #[serde(default)]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub pool_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(alias = "asset0__in", serialize_with = "serialize_comma_separated")]
    pub asset0_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(alias = "asset1__in", serialize_with = "serialize_comma_separated")]
    pub asset1_address__in: HashSet<H256>,

    #[serde(default)]
    #[serde(serialize_with = "serialize_comma_separated")]
    pub assets__in: HashSet<Address>,
}
