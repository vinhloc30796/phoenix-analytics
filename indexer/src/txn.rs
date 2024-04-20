use anyhow::Result;
use log::{debug, error, info};

use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, EncodedTransactionWithStatusMeta,
    TransactionConfirmationStatus,
};

use kafka::producer::{Producer, Record};
use serde::Serialize;

#[derive(Debug, Serialize)]
enum ConfirmationStatus {
    Confirmed,
    Unconfirmed,
    Finalized,
}

pub struct RawTransaction {
    pub confirmed_txn: RpcConfirmedTransactionStatusWithSignature,
    pub encoded_txn: EncodedTransactionWithStatusMeta,
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    signature: String,
    // Send the timestamp in milliseconds (e.g. to Iceberg)
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
        let ms_timestamp = confirmed_txn.block_time.unwrap() as u64 * 1_000; // UNIX mseconds
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
            timestamp: ms_timestamp,
            successful,
            confirmation_status,
            slot,
            fee,
            compute_units,
        })
    }
}

impl Transaction {
    pub fn publish(&self, producer: &mut Producer) -> Result<()> {
        debug!("Publishing transaction: {}", self.signature);
        let record = Record::from_value(
            "transaction",
            serde_json::to_vec(self).unwrap(),
        );
        match producer.send(&record) {
            Ok(_) => {
                info!("Published transaction: {}", self.signature);
                Ok(())
            }
            Err(e) => {
                error!("Error: {:?}", e);
                Err(anyhow::anyhow!("Error: {:?}", e))
            }
        }
    }
}
