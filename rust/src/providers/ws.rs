use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use futures::{
    channel::mpsc, select_biased, stream::Fuse, FutureExt, SinkExt, StreamExt, TryStreamExt,
};
use http::header;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{error, warn};
use tungstenite::{client::IntoClientRequest, Message};
use uuid::Uuid;

use crate::{
    core::{
        error::{Error, ResponseError, Result},
        provider::{ChainProvider, Provider, StreamResponse, UniswapV2Provider, UniswapV3Provider},
        types::format::Format,
    },
    provider::{BtcProvider, CurveProvider, Erc20Provider, FuelProvider},
    requests::{
        self,
        blocks::GetBlocksRequest,
        btc::{GetBtcBlocksRequest, GetBtcTxsRequest},
        erc20::{GetErc20ApprovalsRequest, GetErc20Request, GetErc20TransferssRequest},
        fuel::{
            GetFuelBlocksRequest, GetFuelLogsRequest, GetFuelMessagesRequest,
            GetFuelReceiptsRequest, GetFuelTxsRequest, GetSparkMarketRequest, GetSparkOrderRequest,
            GetSrc20, GetSrc7, GetUtxoRequest,
        },
        logs::GetLogsRequest,
        transfers::GetTransfersRequest,
        txs::GetTxsRequest,
        uniswap_v2::{GetPairsRequest, GetPricesRequest as GetUniswapV2PricesRequest},
        uniswap_v3::{GetPoolsRequest, GetPricesRequest as GetUniswapV3PricesRequest},
    },
    ChainId,
};

const WS_PATH: &str = "v1/websocket";

type WsResult = Result<Vec<u8>>;
type OperationMsg = (
    Uuid,
    Operation,
    Format,
    bool,
    mpsc::UnboundedSender<WsResult>,
);

#[derive(Clone, Debug)]
pub struct WsProvider {
    operations: mpsc::UnboundedSender<OperationMsg>,
}

impl WsProvider {
    async fn request(
        &self,
        operation: Operation,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        let (sink, stream) = mpsc::unbounded();
        let id = Uuid::new_v4();

        self.operations
            .unbounded_send((id, operation, format, deltas, sink))
            .map_err(|_| Error::BackendShutDown)?;

        let stream = stream
            .map_err(Error::from)
            .filter_map(|data| async {
                match data {
                    Ok(data) if !data.is_empty() => Some(Ok(data)),
                    Ok(_) => None,
                    Err(e) => Some(Err(e)),
                }
            })
            .boxed();

        Ok(stream)
    }

    /// Returns true if the WS connection is active, false otherwise
    pub fn ready(&self) -> bool {
        !self.operations.is_closed()
    }
}

#[async_trait]
impl Provider for WsProvider {
    async fn try_new(
        endpoint: String,
        is_secure: bool,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self> {
        let mut req = format!(
            "{}://{endpoint}/{WS_PATH}",
            if is_secure { "wss" } else { "ws" },
        )
        .into_client_request()?;

        if let (Some(username), Some(password)) = (username, password) {
            let auth = format!("{username}:{password}");
            let encoded = BASE64.encode(auth);

            req.headers_mut().append(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Basic {encoded}"))
                    .expect("Only non-ascii chars result in an error"),
            );
        }

        let (sink, stream) = mpsc::unbounded();
        let bw = BackgroundWorker::new(req, stream).await?;
        tokio::spawn(bw.main_loop());

        Ok(Self { operations: sink })
    }

    async fn get_status_by_format(&self, format: Format) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetStatus, format, false).await
    }
}

#[async_trait]
impl ChainProvider for WsProvider {
    async fn get_blocks_by_format(
        &self,
        request: GetBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetBlocks { params: request }, format, deltas)
            .await
    }

    async fn get_logs_by_format(
        &self,
        request: GetLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetLogs { params: request }, format, deltas)
            .await
    }

    async fn get_txs_by_format(
        &self,
        request: GetTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetTxs { params: request }, format, deltas)
            .await
    }

    async fn get_transfers_by_format(
        &self,
        request: GetTransfersRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetTransfers { params: request }, format, deltas)
            .await
    }
}

#[async_trait]
impl UniswapV2Provider for WsProvider {
    async fn get_pairs_by_format(
        &self,
        request: GetPairsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetUniswapV2Pairs { params: request },
            format,
            deltas,
        )
        .await
    }

    async fn get_prices_by_format(
        &self,
        request: requests::uniswap_v2::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetUniswapV2Prices { params: request },
            format,
            deltas,
        )
        .await
    }
}

#[async_trait]
impl UniswapV3Provider for WsProvider {
    async fn get_pools_by_format(
        &self,
        request: GetPoolsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetUniswapV3Pools { params: request },
            format,
            deltas,
        )
        .await
    }

    async fn get_prices_by_format(
        &self,
        request: requests::uniswap_v3::GetPricesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetUniswapV3Prices { params: request },
            format,
            deltas,
        )
        .await
    }
}

#[async_trait]
impl CurveProvider for WsProvider {
    async fn get_tokens_by_format(
        &self,
        request: requests::curve::GetCrvTokenRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetCurveTokens { params: request },
            format,
            deltas,
        )
        .await
    }

    async fn get_pools_by_format(
        &self,
        request: requests::curve::GetCrvPoolRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetCurvePools { params: request }, format, deltas)
            .await
    }

    async fn get_prices_by_format(
        &self,
        request: requests::curve::GetCrvPriceRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetCurvePrices { params: request },
            format,
            deltas,
        )
        .await
    }
}

#[async_trait]
impl Erc20Provider for WsProvider {
    async fn get_erc20_by_format(
        &self,
        request: GetErc20Request,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetErc20 { params: request }, format, deltas)
            .await
    }

    async fn get_erc20_approval_by_format(
        &self,
        request: GetErc20ApprovalsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetErc20Approvals { params: request },
            format,
            deltas,
        )
        .await
    }

    async fn get_erc20_transfers_by_format(
        &self,
        request: GetErc20TransferssRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetErc20Transfers { params: request },
            format,
            deltas,
        )
        .await
    }
}

#[async_trait]
impl FuelProvider for WsProvider {
    async fn get_fuel_blocks_by_format(
        &self,
        request: GetFuelBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetFuelBlocks { params: request }, format, deltas)
            .await
    }

    async fn get_fuel_logs_by_format(
        &self,
        request: GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetFuelLogs { params: request }, format, deltas)
            .await
    }

    async fn get_fuel_logs_decoded_by_format(
        &self,
        request: GetFuelLogsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetFuelLogsDecoded { params: request },
            format,
            deltas,
        )
        .await
    }

    async fn get_fuel_txs_by_format(
        &self,
        request: GetFuelTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetFuelTxs { params: request }, format, deltas)
            .await
    }

    async fn get_fuel_receipts_by_format(
        &self,
        request: GetFuelReceiptsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetFuelReceipts { params: request },
            format,
            deltas,
        )
        .await
    }

    async fn get_fuel_messages_by_format(
        &self,
        request: GetFuelMessagesRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetMessages { params: request }, format, deltas)
            .await
    }

    async fn get_fuel_unspent_utxos_by_format(
        &self,
        request: GetUtxoRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetFuelUnspentUtxos { params: request },
            format,
            deltas,
        )
        .await
    }

    async fn get_fuel_spark_markets_by_format(
        &self,
        request: GetSparkMarketRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(
            Operation::GetSparkMarket { params: request },
            format,
            deltas,
        )
        .await
    }

    async fn get_fuel_spark_orders_by_format(
        &self,
        request: GetSparkOrderRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetSparkOrder { params: request }, format, deltas)
            .await
    }

    async fn get_fuel_src20_by_format(
        &self,
        request: GetSrc20,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetSrc20 { params: request }, format, deltas)
            .await
    }

    async fn get_fuel_src7_by_format(
        &self,
        request: GetSrc7,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        self.request(Operation::GetSrc7 { params: request }, format, deltas)
            .await
    }
}

#[async_trait]
impl BtcProvider for WsProvider {
    async fn get_btc_blocks_by_format(
        &self,
        mut request: GetBtcBlocksRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::BTC]);
        self.request(Operation::GetBtcBlocks { params: request }, format, deltas)
            .await
    }

    async fn get_btc_txs_by_format(
        &self,
        mut request: GetBtcTxsRequest,
        format: Format,
        deltas: bool,
    ) -> StreamResponse<Vec<u8>> {
        request.chains = HashSet::from_iter(vec![ChainId::BTC]);
        self.request(Operation::GetBtcTxs { params: request }, format, deltas)
            .await
    }
}

struct BackgroundWorker {
    ws: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    operations: Fuse<mpsc::UnboundedReceiver<OperationMsg>>,
    subscriptions: HashMap<Uuid, mpsc::UnboundedSender<Result<Vec<u8>>>>,
}

impl BackgroundWorker {
    pub async fn new(
        ws_server: http::Request<()>,
        operations: mpsc::UnboundedReceiver<OperationMsg>,
    ) -> Result<Self> {
        let (ws, _) = connect_async(ws_server).await?;

        Ok(Self {
            ws,
            operations: operations.fuse(),
            subscriptions: HashMap::default(),
        })
    }

    pub async fn main_loop(mut self) {
        let Err(err) = self.try_run().await else {
            return;
        };

        error!("Websocket connection failed: {err}");

        let err = err.to_string();
        for sub in self.subscriptions.values_mut() {
            let _ = sub.send(Err(Error::ErrorMsg(err.clone()))).await;
        }
    }

    async fn try_run(&mut self) -> Result<()> {
        let mut ping_interval = tokio::time::interval(Duration::from_secs(5));
        let mut latest_msg_stamp = Instant::now();

        loop {
            select_biased! {
                _ = ping_interval.tick().fuse() => {
                    if latest_msg_stamp.elapsed().as_secs_f64() > 9.0 {
                        return Err(Error::PingTimeout);
                    }
                    self.ws.send(Message::Ping(Vec::new())).await?
                }
                operation = self.operations.next() => {
                    match operation {
                        Some(operation) => self.operate(operation).await?,
                        None => {
                            self.ws.close(None).await?;
                            return Ok(())
                        },
                    }
                }
                resp = self.ws.try_next() => {
                    match resp {
                        Ok(Some(message)) => {
                            latest_msg_stamp = Instant::now();
                            self.handle(message).await?;
                        }
                        Ok(None) => return Err(Error::UnexpectedClose),
                        Err(e) => return Err(Error::Tungstenite(e)),
                    }
                }
            }
        }
    }

    async fn operate(&mut self, operation: OperationMsg) -> Result<()> {
        let (id, operation, format, deltas, sink) = operation;

        let request = Request {
            id,
            operation,
            format,
            deltas,
        };
        let payload = serde_json::to_vec(&request)?;

        if self.subscriptions.insert(id, sink).is_some() {
            warn!("Replacing already-registered subscription with id {:?}", id);
        }

        self.ws.send(Message::Binary(payload)).await?;

        Ok(())
    }

    async fn handle(&mut self, resp: Message) -> Result<()> {
        match resp {
            Message::Text(_) => Err(Error::UnexpectedMessage),
            Message::Frame(_) => Ok(()), // Server is allowed to send Raw frames
            Message::Ping(inner) => self.handle_ping(inner).await,
            Message::Pong(_) => Ok(()), // Server is allowed to send unsolicited pongs.
            Message::Close(_) => Err(Error::UnexpectedClose),
            Message::Binary(buf) => self.handle_binary(buf).await,
        }
    }

    async fn handle_ping(&mut self, inner: Vec<u8>) -> Result<()> {
        self.ws.send(Message::Pong(inner)).await?;
        Ok(())
    }

    async fn handle_binary(&mut self, data: Vec<u8>) -> Result<()> {
        let (header, data) = Header::try_from_data(data)?;
        let id = header.id;

        let msg = match header.kind {
            Kind::Start => {
                return Ok(());
            }
            Kind::Continue => Ok(data),
            Kind::ContinueWithError => match data.first() {
                Some(b'{') => match serde_json::from_slice::<ResponseError>(&data) {
                    Ok(err) => Err(Error::ErrorResponse(err)),
                    Err(_) => Err(Error::UnexpectedMessageFormat),
                },
                _ => match String::from_utf8(data) {
                    Ok(s) => Err(Error::ErrorMsg(s)),
                    Err(_) => Err(Error::UnexpectedMessageFormat),
                },
            },
            Kind::End => {
                let sink = self.subscriptions.remove(&header.id.0);
                if let Some(sink) = sink {
                    sink.close_channel();
                }
                return Ok(());
            }
            Kind::Error => match String::from_utf8(data) {
                Ok(s) => Err(Error::ErrorMsg(s)),
                Err(_) => Err(Error::UnexpectedMessageFormat),
            },
            _ => Err(Error::UnexpectedMessageFormat),
        };

        if let std::collections::hash_map::Entry::Occupied(occupied) =
            self.subscriptions.entry(id.0)
        {
            if let Err(err) = occupied.get().unbounded_send(msg) {
                if err.is_disconnected() {
                    // subscription channel was closed on the receiver end
                    occupied.remove();
                }
                return Err(Error::Custom(
                    format!("failed to send message: {err:?}").into(),
                ));
            }
        }

        Ok(())
    }
}

#[derive(Clone, serde::Serialize)]
struct Request {
    id: Uuid,
    #[serde(flatten)]
    operation: Operation,
    #[serde(default)]
    format: Format,
    #[serde(default)]
    deltas: bool,
}

#[derive(Clone, serde::Serialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
enum Operation {
    GetStatus,
    GetBlocks {
        #[serde(flatten)]
        params: GetBlocksRequest,
    },
    GetLogs {
        #[serde(flatten)]
        params: GetLogsRequest,
    },
    #[serde(rename = "getDecodedLogs")]
    GetFuelLogsDecoded {
        #[serde(flatten)]
        params: GetFuelLogsRequest,
    },
    GetTxs {
        #[serde(flatten)]
        params: GetTxsRequest,
    },
    #[serde(rename = "getBlocks")]
    GetBtcBlocks {
        #[serde(flatten)]
        params: GetBtcBlocksRequest,
    },
    #[serde(rename = "getTxs")]
    GetBtcTxs {
        #[serde(flatten)]
        params: GetBtcTxsRequest,
    },
    #[serde(rename = "getBlocks")]
    GetFuelBlocks {
        #[serde(flatten)]
        params: GetFuelBlocksRequest,
    },
    #[serde(rename = "getLogs")]
    GetFuelLogs {
        #[serde(flatten)]
        params: GetFuelLogsRequest,
    },
    #[serde(rename = "getTxs")]
    GetFuelTxs {
        #[serde(flatten)]
        params: GetFuelTxsRequest,
    },
    #[serde(rename = "getReceipts")]
    GetFuelReceipts {
        #[serde(flatten)]
        params: GetFuelReceiptsRequest,
    },
    #[serde(rename = "getMessages")]
    GetMessages {
        #[serde(flatten)]
        params: GetFuelMessagesRequest,
    },
    #[serde(rename = "getUnspentUtxos")]
    GetFuelUnspentUtxos {
        #[serde(flatten)]
        params: GetUtxoRequest,
    },
    GetUniswapV2Pairs {
        #[serde(flatten)]
        params: GetPairsRequest,
    },
    GetUniswapV2Prices {
        #[serde(flatten)]
        params: GetUniswapV2PricesRequest,
    },
    GetUniswapV3Pools {
        #[serde(flatten)]
        params: GetPoolsRequest,
    },
    GetUniswapV3Prices {
        #[serde(flatten)]
        params: GetUniswapV3PricesRequest,
    },
    GetCurveTokens {
        #[serde(flatten)]
        params: requests::curve::GetCrvTokenRequest,
    },
    GetCurvePools {
        #[serde(flatten)]
        params: requests::curve::GetCrvPoolRequest,
    },
    GetCurvePrices {
        #[serde(flatten)]
        params: requests::curve::GetCrvPriceRequest,
    },
    GetTransfers {
        #[serde(flatten)]
        params: requests::transfers::GetTransfersRequest,
    },
    GetErc20 {
        #[serde(flatten)]
        params: requests::erc20::GetErc20Request,
    },
    GetErc20Approvals {
        #[serde(flatten)]
        params: requests::erc20::GetErc20ApprovalsRequest,
    },
    GetErc20Transfers {
        #[serde(flatten)]
        params: requests::erc20::GetErc20TransferssRequest,
    },
    GetSparkMarket {
        #[serde(flatten)]
        params: requests::fuel::GetSparkMarketRequest,
    },
    GetSparkOrder {
        #[serde(flatten)]
        params: requests::fuel::GetSparkOrderRequest,
    },
    GetSrc20 {
        #[serde(flatten)]
        params: requests::fuel::GetSrc20,
    },
    GetSrc7 {
        #[serde(flatten)]
        params: requests::fuel::GetSrc7,
    },
}

#[derive(Debug, Clone, Deserialize)]
struct Header {
    pub kind: Kind,
    pub id: MsgId,
    #[serde(rename = "counter")]
    pub _counter: u32,
    #[serde(rename = "epoch")]
    pub _epoch: Option<u64>,
}

impl Header {
    fn try_from_data(mut data: Vec<u8>) -> Result<(Self, Vec<u8>)> {
        // seperate by new line
        let mut split = data.splitn_mut(2, |b| *b == b'\n');
        let header = split.next().ok_or_else(|| Error::UnexpectedMessageFormat)?;
        let data = split.next().ok_or_else(|| Error::UnexpectedMessageFormat)?;
        let header = serde_json::from_slice::<Header>(header)?;
        Ok((header, data.to_vec()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Kind {
    Start,
    Continue,
    ContinueWithError,
    End,
    Error,
    Subscription,
}

/// An id describing a subscription or a response
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MsgId(pub Uuid);
