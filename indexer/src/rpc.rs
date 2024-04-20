use anyhow::Result;
use log::{debug, info};
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_config::{RpcBlockConfig, RpcTransactionConfig},
    rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_program::pubkey::Pubkey;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransactionWithStatusMeta,
    TransactionDetails, UiTransactionEncoding,
};
use std::str::FromStr;

#[allow(dead_code)]
pub fn get_block_transactions(
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

pub fn get_transaction(
    client: &RpcClient,
    signature: &str,
) -> Result<EncodedConfirmedTransactionWithStatusMeta> {
    // str to bytes to signature
    let s = Signature::from_str(signature)?;
    // let encoded_txn = client.get_transaction(&s, UiTransactionEncoding::Json)?;
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    let encoded_txn = client.get_transaction_with_config(&s, config)?;
    debug!("Got transaction for signature: {}", signature);
    Ok(encoded_txn)
}

pub fn get_account_signatures(
    client: &RpcClient,
    pubkey: &str,
    limit: Option<usize>,
) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>> {
    let config = GetConfirmedSignaturesForAddress2Config {
        before: None,
        until: None,
        limit: limit,
        commitment: None,
    };
    let p = Pubkey::from_str(pubkey).unwrap();
    return match client.get_signatures_for_address_with_config(&p, config) {
        Ok(signatures) => {
            debug!("{} signatures found", signatures.len());
            Ok(signatures)
        },
        Err(e) => Err(anyhow::anyhow!("Error: {:?}", e)),
    };
}
