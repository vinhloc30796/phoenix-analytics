use crate::txn::{RawTransaction, Transaction};
use anyhow::Result;
use log::info;
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_config::RpcBlockConfig,
};
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;
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
    let encoded_txn = client.get_transaction(&s, UiTransactionEncoding::Json)?;
    Ok(encoded_txn)
}

pub fn get_account_transactions(
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
    let p = Pubkey::from_str(pubkey).unwrap();
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
