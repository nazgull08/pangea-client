use futures::StreamExt;
use pangea_client::{
    provider::ChainProvider, query::Bound, requests::logs::GetLogsRequest, ClientBuilder, Format,
    core::types::ChainId,
    WsProvider,
};
use std::collections::HashSet;
use ethers_core::types::TxHash;
use ethers_core::utils::keccak256;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = std::env::var("PANGEA_URL").unwrap_or("app.pangea.foundation".to_string());
    let username = std::env::var("PANGEA_USERNAME").unwrap();
    let password = std::env::var("PANGEA_PASSWORD").unwrap();

    println!("");
    println!("URL: \x1b[1m{url}\x1b[0m");
    println!("Username: \x1b[1m{username}\x1b[0m");
    println!("");

    let client = ClientBuilder::default()
        .endpoint(&url)
        .credential(&username, &password)
        .build::<WsProvider>()
        .await
        .unwrap();

    // historic request
    // let request = GetLogsRequest {
    //    from_block: Bound::Exact(10049027), // wen luna was deployed
    //    to_block: Bound::Exact(14732821),   // EOD 7th May 2022
    //    ..Default::default()
    // };

    // historic request (last 1000 blocks) + stream realtime
    // let request = GetLogsRequest {
    //    from_block: Bound::FromLatest(1000),
    //    to_block: Bound::Subscribe,
    //    ..Default::default()
    // };

    // stream realtime
    let request = GetLogsRequest {
        from_block: Bound::FromLatest(1000),
        to_block: Bound::Subscribe,
        // address__in: HashSet::from(["cbf2af75f33a36afa29870274c8e6893a8d0d806".parse().unwrap()]),
        topic0__in: HashSet::from([
            TxHash::from(keccak256("AddLiquidity(address,uint256[2],uint256[2],uint256,uint256)")), // curve 3pool liquidity event
        ]),
        chains: HashSet::from([ChainId::ETH]), // EVM-compatible chains can be specified here
        ..Default::default()
    };

    let stream = client
        .get_logs_by_format(request, Format::JsonStream, false)
        .await
        .unwrap();
    futures::pin_mut!(stream);

    while let Some(Ok(data)) = stream.next().await {
        let data = String::from_utf8(data).unwrap();
        println!("data: {data}");
    }
}
