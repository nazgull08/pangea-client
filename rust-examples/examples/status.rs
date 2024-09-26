use futures::StreamExt;
use pangea_client::{
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

    let client = ClientBuilder::default()
        .endpoint(&url)
        .credential(&username, &password)
        .build::<WsProvider>()
        .await
        .unwrap();

    let mut status = client.get_status().await.unwrap();
    while let Some(status) = status.next().await {
        println!("{:?}", status);
    }
}
