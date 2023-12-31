use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{EncodedTransaction, TransactionDetails, UiTransactionEncoding};
use core::panic;
use std::path::PathBuf;

use log::{debug, error, info};

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
    match get_env() {
        Environment::Devnet => "https://api.devnet.solana.com".to_string(),
        Environment::MainnetBeta => "https://api.mainnet-beta.solana.com".to_string(),
    }
}

// Return something that can be converted into a PathBuf
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

fn main() {
    env_logger::init(); // Initialize the logger
    let client = RpcClient::new(get_rpc_addr()); // Initialize the RPC client

    // Get market printlnrmation
    let market = get_market();
    info!(
        "Market: {}-{} with address {}",
        market["base_ticker"], market["quote_ticker"], market["market"]
    );

    // let block_hash = client.get_latest_blockhash().unwrap();
    let block_hash = "F5akWYT3joJYR6NCbM8sRBTyu4qN9Yi3X52LFDEjafbE";
    let slot = 266_666_666;
    let block = client
        .get_block_with_config(
            slot,
            RpcBlockConfig {
                encoding: Some(UiTransactionEncoding::JsonParsed),
                transaction_details: Some(TransactionDetails::Full),
                rewards: None,
                commitment: None,
                max_supported_transaction_version: Some(0u8),
            },
        )
        .unwrap();
    assert_eq!(block.blockhash, block_hash, "Block hash mismatch");

    // Loop through transactions & print out signatures
    let transactions = block.transactions.unwrap();
    let n_txn = transactions.len();
    if n_txn == 0 {
        info!("No transactions in block");
        return;
    }
    info!("Transactions in latest block:");
    transactions
        .into_iter()
        .enumerate()
        .for_each(|(i, tx)| {
            let cur_tx = tx.to_owned();
            let meta = cur_tx.meta.unwrap();
            match cur_tx.transaction {
                EncodedTransaction::Json(details) => {
                    info!("{} - Transaction ID: {:?}", i, details.signatures[0]);
                    info!("- Meta.Fee: {:?}", meta.fee);
                    info!("- Meta.Compute: {:?}", meta.compute_units_consumed);
                    info!("- Signatures: {:?}\n", details.signatures);
                }
                _ => {
                    let msg = format!("Error: Transaction {} is not in JSON format", i);
                    error!("{}", msg);
                    panic!("{}", msg);
                }
            }
        });
}
