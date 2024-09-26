use ethers_core::types::U256;
use futures::StreamExt;
use pangea_client::{
    provider::ChainProvider, query::Bound, requests::txs::GetTxsRequest, ClientBuilder, Format,
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

    let client = ClientBuilder::default()
        .endpoint(&url)
        .credential(&username, &password)
        .build::<WsProvider>()
        .await
        .unwrap();

    let request = GetTxsRequest {
        from_block: Bound::Latest,
        to_block: Bound::Subscribe,
        value__gte: Some(U256::from(1)), // filter transactions where the `gwei` value is greater than 0
        ..Default::default()
    };

    let stream = client
        .get_txs_by_format(request, Format::JsonStream, false)
        .await
        .unwrap();
    futures::pin_mut!(stream);

    while let Some(Ok(data)) = stream.next().await {
        let data = String::from_utf8(data).unwrap();
        println!("data: {data}");
    }
}
