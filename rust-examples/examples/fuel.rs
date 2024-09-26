use futures::StreamExt;
use pangea_client::{
    provider::FuelProvider,
    query::Bound,
    requests::fuel::{
        // GetFuelBlocksRequest,
        // GetFuelLogsRequest,
        GetFuelReceiptsRequest,
    },
    ClientBuilder,
    Format,
    WsProvider,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = std::env::var("PANGEA_URL").unwrap_or("fuel.beta.pangea.foundation".to_string());
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

    // stream realtime
    // let request = GetFuelBlocksRequest {
    // let request = GetFuelLogsRequest {
    // let request = GetFuelReceiptsRequest {
    //    from_block: Bound::Latest,
    //    to_block: Bound::Subscribe,
    //    ..Default::default()
    // };

    // historic request (last 100 blocks) + stream realtime
    // let request = GetFuelBlocksRequest {
    // let request = GetFuelLogsRequest {
    // let request = GetFuelReceiptsRequest {
    //    from_block: Bound::FromLatest(100),
    //    to_block: Bound::Subscribe,
    //    ..Default::default()
    // };

    // historic request -- last 100 blocks
    // let request = GetFuelBlocksRequest {
    // let request = GetFuelLogsRequest {
    let request = GetFuelReceiptsRequest {
        from_block: Bound::FromLatest(100),
        to_block: Bound::Latest,
        ..Default::default()
    };

    let stream = client
        // .get_fuel_blocks_by_format(request, Format::JsonStream, false)
        // .get_fuel_logs_by_format(request, Format::JsonStream, false)
        .get_fuel_receipts_by_format(request, Format::JsonStream, false)
        .await
        .expect("Failed to get fuel blocks");
    futures::pin_mut!(stream);

    while let Some(data) = stream.next().await {
        let data = data.unwrap();
        let data = String::from_utf8(data).unwrap();
        println!("data: {data:?}");
    }
}
