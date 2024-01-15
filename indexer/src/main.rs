use anyhow::Result;
use indexer::pubsub::init_producer;
use indexer::rpc::{get_account_signatures, get_transaction};
use indexer::txn::{RawTransaction, Transaction};
use log::{debug, error, info};
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction};
use std::path::PathBuf;

enum Environment {
    Devnet,
    MainnetBeta,
}

fn get_env() -> Environment {
    match std::env::var("ENV")
        .unwrap_or("devnet".to_string())
        .as_str()
    {
        "devnet" => Environment::Devnet,
        "mainnet-beta" => Environment::MainnetBeta,
        _ => Environment::Devnet,
    }
}

fn get_rpc_addr() -> String {
    // Check if the RPC_URL environment variable is set
    let addr = match get_env() {
        Environment::Devnet => "https://api.devnet.solana.com".to_string(),
        Environment::MainnetBeta => "https://api.mainnet-beta.solana.com".to_string(),
    };
    debug!("Using RPC URL: {}", addr);
    addr
}

// Get the market information
fn get_market() -> serde_json::Value {
    let filename = match get_env() {
        Environment::Devnet => "devnet_markets.json",
        Environment::MainnetBeta => "mainnet_markets.json",
    };
    let filepath: PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].iter().collect();
    // Print the path
    debug!("Using market file: {:?}", filepath);

    // Read the file to get the dict
    let file = std::fs::File::open(filepath).unwrap();
    let dict: serde_json::Value = serde_json::from_reader(file).unwrap();
    // Return the first element
    dict[0].clone()
}

#[allow(dead_code)]
fn print_transactions(transactions: Vec<EncodedConfirmedTransactionWithStatusMeta>) {
    transactions.into_iter().enumerate().for_each(|(i, tx)| {
        let inner_tx = tx.transaction.transaction;
        match inner_tx {
            EncodedTransaction::Json(details) => {
                let meta = tx.transaction.meta.unwrap();
                info!("{} - Transaction ID: {:?}", i, details.signatures);
                info!("- Meta.Fee: {:?}", meta.fee);
                info!("- Meta.Compute: {:?}", meta.compute_units_consumed);
                info!("- Signatures: {:?}\n", details.signatures);
            }
            _ => error!("Error: Transaction is not in JSON format"),
        }
    })
}

fn main() -> Result<()> {
    env_logger::init(); // Initialize the logger
    let client = RpcClient::new(get_rpc_addr()); // Initialize the RPC client

    // Get market printlnrmation
    let market = get_market();
    info!(
        "Market: {}-{} with address {}",
        market["base_ticker"], market["quote_ticker"], market["market"]
    );

    // let block_hash = client.get_latest_blockhash().unwrap();
    // Extract data from the market
    let market_pubkey = match market["market"].as_str() {
        Some(s) => s,
        None => return Err(anyhow::anyhow!("Error: Failed to get market address")),
    };
    let signatures = get_account_signatures(&client, market_pubkey, Some(10)).unwrap();
    let raw_txns = signatures
        .iter()
        .map(|signature| {
            let raw_transaction = get_transaction(&client, &signature.signature).unwrap();
            return raw_transaction;
        })
        .collect::<Vec<_>>();
    let transactions: Vec<Transaction> = signatures
        .iter()
        .zip(raw_txns.iter())
        .map(|(signature, raw_transaction)| {
            Transaction::try_from(RawTransaction {
                confirmed_txn: signature.clone(),
                encoded_txn: raw_transaction.transaction.clone(),
            })
            .unwrap()
        })
        .collect();
    info!("{:?}", transactions);

    // Publish
    let mut producer = init_producer();
    transactions
        .iter()
        .for_each(|tx| tx.publish(&mut producer).unwrap());
    Ok(())
}
