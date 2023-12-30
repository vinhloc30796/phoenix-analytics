use solana_rpc_client::rpc_client::RpcClient;
use solana_transaction_status::{EncodedTransaction, UiTransactionEncoding};

fn get_rpc_addr() -> String {
    // Check if the RPC_URL environment variable is set
    if let Ok(rpc_addr) = std::env::var("RPC_URL") {
        return rpc_addr;
    }
    return "http://localhost:8899".to_string();
}

fn main() {
    let client = RpcClient::new(get_rpc_addr());
    let block_hash = client.get_latest_blockhash().unwrap();
    let slot = match client.get_highest_snapshot_slot() {
        Ok(slot) => slot,
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    };
    println!(
        "Latest block hash: {:?}; Snapshot slot: {:?}",
        block_hash, slot
    );
    let latest_block = client
        .get_block_with_encoding(slot.full, UiTransactionEncoding::Json)
        .unwrap();
    match latest_block.block_height {
        Some(height) => {
            assert_eq!(height, slot.full, "Block height mismatch");
            println!("Latest block height: {:?}", height);
            height
        }
        None => {
            println!("Latest block height: None");
            panic!("Block height is None");
        }
    };

    // Loop through transactions & print out signatures
    println!("Transactions in latest block:");
    for tx in latest_block.transactions {
        println!("- Meta.Fee: {:?}", tx.meta.unwrap().fee);
        match tx.transaction {
            EncodedTransaction::Json(details) => {
                println!("- Message: {:?}", details.message);
                println!("- Signatures: {:?}", details.signatures);
            }
            _ => {
                println!("Error: Transaction is not in JSON format");
            }
        }
    }
}
