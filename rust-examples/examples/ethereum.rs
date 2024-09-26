use futures::StreamExt;
use pangea_client::{
    provider::ChainProvider, query::Bound, requests::blocks::GetBlocksRequest, ClientBuilder,
    core::types::ChainId,
    Format,
    WsProvider,
};
use std::collections::HashSet;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = std::env::var("PANGEA_URL").unwrap_or("beta.pangea.foundation".to_string());
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

    // historic request -- 1 block
    // let request = GetBlocksRequest {
    //    from_block: Bound::Exact(17_000_000),
    //    to_block: Bound::Exact(17_001_000),
    //    ..Default::default()
    // }

    // historic request -- last 100 blocks
    // let request = GetSparkOrderRequest {
    //    from_block: Bound::FromLatest(100),
    //    to_block: Bound::Latest,
    //    ..Default::default()
    // };

    // stream realtime
    let request = GetBlocksRequest {
        from_block: Bound::Latest,
        to_block: Bound::Subscribe,
        chains: HashSet::from([ChainId::ETH]), // EVM-compatible chains can be specified here
        ..Default::default()
    };

    let stream = client
        .get_blocks_by_format(request, Format::JsonStream, false)
        .await
        .unwrap();
    futures::pin_mut!(stream);

    while let Some(Ok(data)) = stream.next().await {
        let data = String::from_utf8(data).unwrap();
        println!("data: {data}");
    }
}
