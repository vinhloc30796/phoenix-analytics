use anyhow::Result;
use core::panic;
use log::{debug, error, info};
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_config::RpcBlockConfig,
    rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_program::pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta,
    EncodedTransaction, EncodedTransactionWithStatusMeta, TransactionConfirmationStatus,
    TransactionDetails, UiTransactionEncoding,
};
use std::{path::PathBuf, str::FromStr};

enum Environment {
    Devnet,
    MainnetBeta,
}

#[derive(Debug)]
enum ConfirmationStatus {
    Confirmed,
    Unconfirmed,
    Finalized,
}

struct RawTransaction {
    confirmed_txn: RpcConfirmedTransactionStatusWithSignature,
    encoded_txn: EncodedTransactionWithStatusMeta,
}

#[derive(Debug)]
struct Transaction {
    signature: String,
    timestamp: u64,
    successful: bool,
    confirmation_status: ConfirmationStatus,
    slot: u64,
    fee: u64,
    compute_units: u64,
}

impl TryFrom<RawTransaction> for Transaction {
    type Error = anyhow::Error;

    fn try_from(raw_txn: RawTransaction) -> Result<Self> {
        // Deconstruct the raw transaction
        let confirmed_txn = raw_txn.confirmed_txn;
        let encoded_txn = raw_txn.encoded_txn;

        // Get the necessary fields from EncodedTransactionWithStatusMeta
        let meta = encoded_txn.meta.unwrap();
        let successful = meta.err.is_none();
        let signature = match encoded_txn.transaction {
            EncodedTransaction::Json(details) => details.signatures[0].to_string(),
            _ => return Err(anyhow::anyhow!("Error: Transaction is not in JSON format")),
        };
        let fee = meta.fee;

        // Get the necessary fields from RpcConfirmedTransactionStatusWithSignature
        let timestamp = confirmed_txn.block_time.unwrap() as u64;
        let confirmation_status = match confirmed_txn.confirmation_status {
            Some(TransactionConfirmationStatus::Confirmed) => ConfirmationStatus::Confirmed,
            Some(TransactionConfirmationStatus::Finalized) => ConfirmationStatus::Finalized,
            Some(TransactionConfirmationStatus::Processed) => ConfirmationStatus::Confirmed,
            None => ConfirmationStatus::Unconfirmed,
        };
        // let confirmations = meta.confirmations.unwrap();
        let slot = confirmed_txn.slot.into();
        let compute_units = match meta.compute_units_consumed {
            OptionSerializer::Some(c) => c,
            OptionSerializer::None => 0,
            OptionSerializer::Skip => panic!("Error: Compute units consumed is None"),
        };
        Ok(Transaction {
            signature,
            timestamp,
            successful,
            confirmation_status,
            slot,
            fee,
            compute_units,
        })
    }
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

fn get_transaction(
    client: &RpcClient,
    signature: &str,
) -> Result<EncodedConfirmedTransactionWithStatusMeta> {
    // str to bytes to signature
    let s = Signature::from_str(signature)?;
    let encoded_txn = client.get_transaction(&s, UiTransactionEncoding::Json)?;
    Ok(encoded_txn)
}

fn get_account_transactions(
    client: &RpcClient,
    pubkey: &str,
    limit: Option<usize>,
) -> Result<Vec<Transaction>> {
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
                Ok(tx) => Ok(Transaction::try_from(RawTransaction {
                    confirmed_txn: signature.clone(),
                    encoded_txn: tx.transaction,
                })
                .unwrap()),
                Err(e) => Err(anyhow::anyhow!("Error: {:?}", e))?,
            },
        )
        .collect::<Result<Vec<Transaction>>>()?;
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
    println!("{:?}", transactions);
    Ok(())
}
