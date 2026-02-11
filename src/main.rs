mod proto {
    tonic::include_proto!("acropolis.indexer.v1");
}

use proto::chain_sync_service_client::ChainSyncServiceClient;
use proto::{GetBlockByHashRequest, GetTipRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "http://127.0.0.1:50051".into());

    println!("Connecting to {addr}...");
    let mut client = ChainSyncServiceClient::connect(addr).await?;
    println!("Connected.\n");

    loop {
        // 1. Get the current tip
        let tip_resp = client.get_tip(GetTipRequest {}).await?.into_inner();

        let Some(tip) = tip_resp.tip else {
            println!("No tip yet, waiting...");
            tokio::time::sleep(Duration::from_secs(2)).await;
            continue;
        };

        println!(
            "Tip: slot={} hash={}",
            tip.slot,
            hex::encode(&tip.block_hash)
        );

        // 2. Ask for that block by hash
        let block_resp = client
            .get_block_by_hash(GetBlockByHashRequest {
                hash: tip.block_hash.clone(),
            })
            .await;

        match block_resp {
            Ok(resp) => {
                if let Some(block) = resp.into_inner().block {
                    println!(
                        "  block_number={} epoch={} slot={} timestamp={} tx_count={}",
                        block.block_number, block.epoch, block.slot, block.timestamp, block.tx_count,
                    );
                }
            }
            Err(status) => {
                println!("  GetBlockByHash: {}", status.message());
            }
        }

        println!();
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
