use anyhow::Result;
use core::panic;
use log::{debug, error, info};
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_config::RpcBlockConfig,
};
use solana_program::pubkey;
use solana_sdk::signature;
use solana_transaction_status::{
    EncodedTransaction, EncodedTransactionWithStatusMeta, TransactionDetails, UiTransactionEncoding,
};
use std::{path::PathBuf, str::FromStr};

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
fn get_block_transactions(
    client: &RpcClient,
    slot: u64,
    expected_blockhash: Option<String>,
) -> Result<Vec<EncodedTransactionWithStatusMeta>> {
    // let block_hash = "F5akWYT3joJYR6NCbM8sRBTyu4qN9Yi3X52LFDEjafbE";
    // let slot = 266_666_666;
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        transaction_details: Some(TransactionDetails::Full),
        rewards: None,
        commitment: None,
        max_supported_transaction_version: Some(0u8),
    };

    let block = client.get_block_with_config(slot, config).unwrap();
    match expected_blockhash {
        Some(expected_blockhash) => {
            assert_eq!(
                block.blockhash, expected_blockhash,
                "Expected blockhash mismatch"
            )
        }
        None => (),
    }

    // Loop through transactions & print out signatures
    let transactions = block.transactions.unwrap();
    let n_txn = transactions.len();
    if n_txn == 0 {
        info!("No transactions in block");
        return Ok(transactions);
    }
    info!("{} transactions in block", n_txn);
    return Ok(transactions);
}

#[allow(dead_code)]
fn print_transactions(transactions: Vec<EncodedTransactionWithStatusMeta>) {
    transactions.into_iter().enumerate().for_each(|(i, tx)| {
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

fn get_transaction(
    client: &RpcClient,
    signature: &str,
) -> Result<EncodedTransactionWithStatusMeta> {
    // str to bytes to signature
    let s = signature::Signature::from_str(signature)?;
    let transaction = client.get_transaction(&s, UiTransactionEncoding::Json)?;
    Ok(transaction.transaction)
}

fn get_account_transactions(
    client: &RpcClient,
    pubkey: &str,
    limit: Option<usize>,
) -> Result<Vec<EncodedTransactionWithStatusMeta>> {
    let config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: limit,
        commitment: None,
    };
    let p = pubkey::Pubkey::from_str(pubkey).unwrap();
    let signatures = client
        .get_signatures_for_address_with_config(&p, config)
        .unwrap();
    let outs = signatures
        .iter()
        .map(
            |signature| match get_transaction(&client, &signature.signature) {
                Ok(tx) => Ok(tx),
                Err(e) => Err(anyhow::anyhow!("Error: {:?}", e))?,
            },
        )
        .collect::<Result<Vec<EncodedTransactionWithStatusMeta>>>()?;
    Ok(outs)
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
    let market_pubkey = match market["market"].as_str() {
        Some(s) => s,
        None => return Err(anyhow::anyhow!("Error: Failed to get market address")),
    };
    let transactions = get_account_transactions(&client, market_pubkey, Some(10)).unwrap();
    print_transactions(transactions);
    Ok(())
}
