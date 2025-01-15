use std::{collections::HashSet, pin::Pin};

use async_trait::async_trait;
use futures::Stream;

use super::{
    error::Result,
    requests::{
        self,
        blocks::GetBlocksRequest,
        btc::{GetBtcBlocksRequest, GetBtcTxsRequest},
        fuel::{GetFuelReceiptsRequest, GetSrc20, GetSrc7, GetUtxoRequest},
        logs::GetLogsRequest,
        txs::GetTxsRequest,
        uniswap_v2::GetPairsRequest,
        uniswap_v3::GetPoolsRequest,
    },
};
use crate::{
    requests::{
        curve::{GetCrvPoolRequest, GetCrvPriceRequest, GetCrvTokenRequest},
        erc20::{GetErc20ApprovalsRequest, GetErc20Request, GetErc20TransferssRequest},
        fuel::{
            GetFuelBlocksRequest, GetFuelLogsRequest, GetFuelTxsRequest, GetSparkMarketRequest,
            GetSparkOrderRequest,
        },
        mira::{GetMiraLiquidityRequest, GetMiraPoolsRequest, GetMiraSwapsRequest},
        transfers::GetTransfersRequest,
    },
    ChainId, Error, Format,
};

pub type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T>> + Send>>;
pub type StreamResponse<T> = Result<ResponseStream<T>>;

#[async_trait]
pub trait Provider: Sized {
    async fn try_new(
        endpoint: String,
        is_secure: bool,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self>;

    async fn get_status_by_format(&self, format: Format) -> StreamResponse<Vec<u8>>;
}

#[async_trait]
pub trait ChainProvider {
    async fn get_blocks_by_format(
        &self,
        request: GetBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
    async fn get_logs_by_format(
        &self,
        request: GetLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
    async fn get_txs_by_format(
        &self,
        request: GetTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_transfers_by_format(
        &self,
        request: GetTransfersRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
}

#[async_trait]
pub trait UniswapV2Provider {
    async fn get_pairs_by_format(
        &self,
        request: GetPairsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
    async fn get_prices_by_format(
        &self,
        request: requests::uniswap_v2::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
}

#[async_trait]
pub trait UniswapV3Provider {
    async fn get_fees_by_format(
        &self,
        request: requests::uniswap_v3::GetFeesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
    async fn get_pools_by_format(
        &self,
        request: GetPoolsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
    async fn get_positions_by_format(
        &self,
        request: requests::uniswap_v3::GetPositionsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
    async fn get_prices_by_format(
        &self,
        request: requests::uniswap_v3::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
}

#[async_trait]
pub trait CurveProvider {
    async fn get_tokens_by_format(
        &self,
        request: GetCrvTokenRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
    async fn get_pools_by_format(
        &self,
        request: GetCrvPoolRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
    async fn get_prices_by_format(
        &self,
        request: GetCrvPriceRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
}

#[async_trait]
pub trait Erc20Provider {
    async fn get_erc20_by_format(
        &self,
        request: GetErc20Request,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_erc20_approval_by_format(
        &self,
        request: GetErc20ApprovalsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_erc20_transfers_by_format(
        &self,
        request: GetErc20TransferssRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
}

#[async_trait]
pub trait FuelProvider {
    const FUEL_VALID_CHAINS: [ChainId; 2] = [ChainId::FUEL, ChainId::FUELTESTNET];

    async fn get_fuel_blocks_by_format(
        &self,
        request: GetFuelBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_logs_by_format(
        &self,
        request: GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_logs_decoded_by_format(
        &self,
        request: GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_txs_by_format(
        &self,
        request: GetFuelTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_receipts_by_format(
        &self,
        request: GetFuelReceiptsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_messages_by_format(
        &self,
        request: requests::fuel::GetFuelMessagesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_unspent_utxos_by_format(
        &self,
        request: GetUtxoRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_spark_markets_by_format(
        &self,
        request: GetSparkMarketRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_spark_orders_by_format(
        &self,
        request: GetSparkOrderRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_src20_by_format(
        &self,
        request: GetSrc20,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_src7_by_format(
        &self,
        request: GetSrc7,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_mira_v1_pools_by_format(
        &self,
        request: GetMiraPoolsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_mira_v1_liquidity_by_format(
        &self,
        request: GetMiraLiquidityRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_fuel_mira_v1_swaps_by_format(
        &self,
        request: GetMiraSwapsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    fn check_chain(&self, chains: &HashSet<ChainId>) -> Result<()> {
        if !chains
            .iter()
            .all(|chain| Self::FUEL_VALID_CHAINS.contains(chain))
        {
            return Err(Error::InvalidChainId(chains.clone()));
        }

        Ok(())
    }
}

#[async_trait]
pub trait BtcProvider {
    async fn get_btc_blocks_by_format(
        &self,
        request: GetBtcBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;

    async fn get_btc_txs_by_format(
        &self,
        request: GetBtcTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>>;
}
