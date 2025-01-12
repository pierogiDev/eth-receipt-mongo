# web3-mongo-types

A Rust library for converting Ethereum transaction receipts from web3 to MongoDB BSON documents.

## Features

- Convert `web3::types::TransactionReceipt` to `mongodb::bson::Document`
- Proper handling of all transaction receipt fields
- Hex encoding for addresses and hashes
- Support for logs and their nested structures
- Null value handling for optional fields

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
web3-mongo-types = { git = "https://github.com/pierogiDev/eth-receipt-mongo" }
```

### Basic Example

```rust
use web3_mongo_types::WrappedTransactionReceipt;
use mongodb::bson::Document;
use web3::types::TransactionReceipt;

// Assuming you have a transaction receipt from web3
let receipt: TransactionReceipt = // ... get your transaction receipt

// Convert to MongoDB Document
let wrapped = WrappedTransactionReceipt::new(receipt);
let doc: Document = wrapped.into();

// Now you can insert the document into MongoDB
collection.insert_one(doc, None).await?;
```

## Field Mappings

The library converts transaction receipt fields as follows:

| Ethereum Field | MongoDB Field | Type |
|---------------|---------------|------|
| transactionHash | transactionHash | String (hex) |
| blockHash | blockHash | String (hex) |
| blockNumber | blockNumber | Int64 |
| transactionIndex | transactionIndex | Int64 |
| from | from | String (hex) |
| to | to | String (hex) |
| cumulativeGasUsed | cumulativeGasUsed | String |
| gasUsed | gasUsed | String |
| contractAddress | contractAddress | String (hex) |
| logs | logs | Array |
| status | status | Int32 |
| logsBloom | logs_bloom | String (hex) |
| type | type | Int64 |
| effectiveGasPrice | effectiveGasPrice | String |

## Requirements

- Rust 2021 edition or later
- Dependencies:
  - mongodb = "3.1.1"
  - web3 = "0.19.0"
  - ethereum-types = "0.15.1"
  - hex = "0.4.3"

## License

This project is licensed under the MIT License.
