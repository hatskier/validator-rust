
use awc::Client;
use bundlr_sdk::verify::{file::verify_file_bundle};
use paris::error;
use serde::{Deserialize, Serialize};

use crate::types::Validator;

use super::error::ValidatorCronError;

#[derive(Default)]
pub struct Bundler {
    address: String,
    url: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct TxReceipt {
    block: u64,
    tx_id: String,
    signature: String
}

pub struct Tx {
    id: String,
    block: u64
}

pub async fn get_bundler() -> Result<Bundler, ValidatorCronError> {
    Ok(Bundler { 
                address: "address1".to_string(),
                url: "url".to_string()
            })
}

pub async fn validate_bundler(bundler: Bundler) -> Result<(), ValidatorCronError> {

    // Get latest txs for bundler
    // TODO: Get tx info
    let txs = Vec::<Tx>::new();

    // For each tx see if I or my peers have the tx in their db
    for tx in txs {
        // TODO: Check seeded
        // TODO: Download bundle

        let bundle_txs = verify_file_bundle("filename".to_string()).await.unwrap();
        for bundle_tx in bundle_txs {
            let tx_receipt = if let Ok(tx_receipt) = tx_exists_in_db(tx.id.as_str()).await {
                tx_receipt
            } else if let Ok(tx_receipt) = tx_exists_on_peers(tx.id.as_str()).await {
                tx_receipt
            } else {
                continue;
            };

            // Verify tx receipt
        }
    };

    // If no - sad

    // If yes - check that block_seeded == block_expected

    // If valid - return

    // If not - vote to slash... once vote is confirmed then tell all peers to check

    Ok(())
}

async fn tx_exists_in_db(tx_id: &str) -> Result<TxReceipt, ValidatorCronError> {
    Ok(TxReceipt::default())
}

async fn tx_exists_on_peers(tx_id: &str) -> Result<TxReceipt, ValidatorCronError> {
    let client = Client::default();
    let validator_peers = Vec::<Validator>::new();
    for peer in validator_peers {
        let response = client
            .get(format!("{}/tx/{}", peer.url, tx_id))
            .send()
            .await;
        
            if let Err(r) = response {
                error!("Error occurred while getting tx from peer - {}", r);
                continue;
            }

        let mut response = response.unwrap();

        if response.status().is_success() {
            return Ok(response
                            .json()
                            .await
                            .unwrap())
        }
    }

    Err(ValidatorCronError::TxNotFound)
}