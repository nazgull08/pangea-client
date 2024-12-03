use arrow::{ipc::reader::StreamReader, util::pretty::print_batches};
use futures::StreamExt;
use pangea_client::{
    core::types::ChainId, provider::ChainProvider, query::Bound,
    requests::blocks::GetBlocksRequest, ClientBuilder, Format, WsProvider,
};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv_override().ok();

    let client = ClientBuilder::default()
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
    // let request = GetBlocksRequest {
    //    from_block: Bound::FromLatest(100),
    //    to_block: Bound::Latest,
    //    ..Default::default()
    // };

    // get latest block
    let request = GetBlocksRequest {
        chains: HashSet::from([ChainId::ETH]), // chains can be specified here
        from_block: Bound::FromLatest(10),
        to_block: Bound::Latest,
        ..Default::default()
    };

    let stream = client
        .get_blocks_by_format(request, Format::Arrow, false)
        .await
        .unwrap();
    futures::pin_mut!(stream);

    while let Some(Ok(data)) = stream.next().await {
        let cursor = std::io::Cursor::new(data);
        let mut reader = StreamReader::try_new(cursor, None)?;
        while let Some(batch) = reader.next() {
            let batch = batch?;
            print_batches(&[batch])?;
        }
    }

    Ok(())
}
