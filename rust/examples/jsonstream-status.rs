use futures::StreamExt;
use pangea_client::{ClientBuilder, WsProvider};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv_override().ok();

    let client = ClientBuilder::default()
        .build::<WsProvider>()
        .await
        .unwrap();

    let mut status = client.get_status().await.unwrap();
    while let Some(status) = status.next().await {
        println!("{:?}", status);
    }

    Ok(())
}
