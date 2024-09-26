use std::collections::HashSet;

use futures::StreamExt;
use pangea_client::{
    core::types::{format::Format, ChainId},
    provider::UniswapV3Provider,
    query::Bound,
    requests::uniswap_v3::GetPricesRequest,
    ClientBuilder,
    WsProvider,
};

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

    // setup websocket client
    let client = ClientBuilder::default()
        .endpoint(&url)
        .credential(&username, &password)
        .build::<WsProvider>()
        .await
        .unwrap();

    // historic request (last 100 blocks)
    // let request = GetPricesRequest {
    //    from_block: Bound::FromLatest(100),
    //    to_block: Bound::Latest,
    //    ..Default::default()
    // };

    // historic request (last 100 blocks) + stream realtime
    // let request = GetPricesRequest {
    //    from_block: Bound::FromLatest(100),
    //    to_block: Bound::Subscribe,
    //    ..Default::default()
    // };

    // stream realtime prices
    let request = GetPricesRequest {
        from_block: Bound::Latest,
        to_block: Bound::Subscribe,
        chains: HashSet::from([ChainId::ETH]),
        ..Default::default()
    };

    let stream = client.get_prices_by_format(request, Format::JsonStream, false).await.unwrap();

    futures::pin_mut!(stream);

    // async iterator over stream of data
    while let Some(data) = stream.next().await {
        let price = String::from_utf8(data.unwrap()).unwrap(); // or use serde json
        println!("Price: {price:?}");
    }
}
