use mongodb::bson::{Bson, Document};
use web3::types::TransactionReceipt;
use std::string::String;
use hex::ToHex;

pub struct WrappedTransactionReceipt(TransactionReceipt);

impl WrappedTransactionReceipt {
    pub fn new(transaction_receipt: TransactionReceipt) -> Self {
        WrappedTransactionReceipt(transaction_receipt)
    }
}

impl From<WrappedTransactionReceipt> for Document {
    fn from(transaction_receipt: WrappedTransactionReceipt) -> Self {
        let mut doc = Document::new();

        doc.insert("transactionHash", Bson::String(format!("0x{}", transaction_receipt.0.transaction_hash.encode_hex::<String>())));
        doc.insert("transactionIndex", Bson::Int64(transaction_receipt.0.transaction_index.as_u64() as i64));
        
        if let Some(block_hash) = transaction_receipt.0.block_hash {
            doc.insert("blockHash", Bson::String(format!("0x{}", block_hash.encode_hex::<String>())));
        } else {
            doc.insert("blockHash", Bson::Null);
        }
        
        if let Some(block_number) = transaction_receipt.0.block_number {
            doc.insert("blockNumber", Bson::Int64(block_number.as_u64() as i64));
        } else {
            doc.insert("blockNumber", Bson::Null);
        }
        
        doc.insert("from", Bson::String(format!("0x{}", transaction_receipt.0.from.encode_hex::<String>())));
        
        if let Some(to) = transaction_receipt.0.to {
            doc.insert("to", Bson::String(format!("0x{}", to.encode_hex::<String>())));
        } else {
            doc.insert("to", Bson::Null);
        }
        
        doc.insert("cumulativeGasUsed", Bson::String(format!("{}", transaction_receipt.0.cumulative_gas_used)));
        
        if let Some(gas_used) = transaction_receipt.0.gas_used {
            doc.insert("gasUsed", Bson::String(format!("{}", gas_used)));
        } else {
            doc.insert("gasUsed", Bson::Null);
        }
        
        if let Some(contract_address) = transaction_receipt.0.contract_address {
            doc.insert("contractAddress", Bson::String(format!("0x{}", contract_address.encode_hex::<String>())));
        } else {
            doc.insert("contractAddress", Bson::Null);
        }

        let logs_doc: Vec<Document> = transaction_receipt.0.logs.into_iter().map(|log| {
            let mut log_doc = Document::new();
            log_doc.insert("address", Bson::String(format!("0x{}", log.address.encode_hex::<String>())));
            log_doc.insert("topics", Bson::Array(log.topics.into_iter().map(|topic| Bson::String(format!("0x{}", topic.encode_hex::<String>()))).collect()));
            log_doc.insert("data", Bson::String(format!("0x{}", log.data.0.encode_hex::<String>())));
            
            if let Some(block_hash) = log.block_hash {
                log_doc.insert("blockHash", Bson::String(format!("0x{}", block_hash.encode_hex::<String>())));
            } else {
                log_doc.insert("blockHash", Bson::Null);
            }
            
            if let Some(block_number) = log.block_number {
                log_doc.insert("blockNumber", Bson::Int64(block_number.as_u64() as i64));
            } else {
                log_doc.insert("blockNumber", Bson::Null);
            }
            
            if let Some(transaction_hash) = log.transaction_hash {
                log_doc.insert("transactionHash", Bson::String(format!("0x{}", transaction_hash.encode_hex::<String>())));
            } else {
                log_doc.insert("transactionHash", Bson::Null);
            }
            
            if let Some(transaction_index) = log.transaction_index {
                log_doc.insert("transactionIndex", Bson::Int64(transaction_index.as_u64() as i64));
            } else {
                log_doc.insert("transactionIndex", Bson::Null);
            }
            
            if let Some(log_index) = log.log_index {
                log_doc.insert("logIndex", Bson::Int64(log_index.as_u64() as i64));
            } else {
                log_doc.insert("logIndex", Bson::Null);
            }
            
            if let Some(log_type) = log.log_type {
                log_doc.insert("type", Bson::String(log_type));
            } else {
                log_doc.insert("type", Bson::Null);
            }
            
            if let Some(removed) = log.removed {
                log_doc.insert("removed", Bson::Boolean(removed));
            } else {
                log_doc.insert("removed", Bson::Null);
            }
            
            log_doc
        }).collect();
        
        let logs_bson: Vec<Bson> = logs_doc.into_iter().map(Bson::Document).collect();
        doc.insert("logs", Bson::Array(logs_bson));
        
        if let Some(status) = transaction_receipt.0.status {
            doc.insert("status", Bson::Int32(status.as_u32() as i32));
        } else {
            doc.insert("status", Bson::Null);
        }
        
        if let Some(root) = transaction_receipt.0.root {
            doc.insert("root", Bson::String(format!("0x{}", root.encode_hex::<String>())));
        } else {
            doc.insert("root", Bson::Null);
        }
        
        doc.insert("logs_bloom", Bson::String(format!("0x{}", transaction_receipt.0.logs_bloom.encode_hex::<String>())));

        if let Some(transaction_type) = transaction_receipt.0.transaction_type {
            doc.insert("type", Bson::Int64(transaction_type.as_u64() as i64));
        } else {
            doc.insert("type", Bson::Null);
        }
        
        if let Some(effective_gas_price) = transaction_receipt.0.effective_gas_price {
            doc.insert("effectiveGasPrice", Bson::String(format!("{}", effective_gas_price)));
        } else {
            doc.insert("effectiveGasPrice", Bson::Null);
        }

        doc
    }
}
