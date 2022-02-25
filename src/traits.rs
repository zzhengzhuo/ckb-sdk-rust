//! The traits defined here is intent to describe the requirements of current
//!  library code and only implemented the trait in upper level code.

use ckb_types::{
    bytes::Bytes,
    packed::{CellOutput, OutPoint, Transaction, Header},
    H256,
};
use thiserror::Error;

/// Wallet errors
#[derive(Error, Debug)]
pub enum WalletError {
    #[error("the id is not found in the wallet")]
    IdNotFound,
    #[error("invalid message, reason: `{0}`")]
    InvalidMessage(String),
    #[error("get transaction dependency failed: `{0}`")]
    TxDep(#[from] TxDepProviderError),
}

/// A wallet abstraction, support wallet type:
///    * secp256k1 ckb wallet
///    * secp256k1 eth wallet
///    * RSA wallet
///    * Hardware wallet
pub trait Wallet {
    // typecial id are blake160(pubkey) and keccak256(pubkey)[12..20]
    fn match_id(&self, id: Bytes) -> bool;

    // `message` type is Bytes, because different algorithm have different length of message.
    //   * secp256k1 => 256bits
    //   * RSA       => 512bits (when key size is 1024bits)
    fn sign(
        &self,
        id: Bytes,
        message: Bytes,
        tx: Transaction,
        // This is mainly for hardware wallet.
        tx_dep_provider: &mut dyn TransactionDependencyProvider,
    ) -> Result<Bytes, WalletError>;

    // Verify a signature
    fn verify(&self, id: Bytes, message: Bytes, signature: Bytes) -> Result<bool, WalletError>;
}

/// Transaction dependency provider errors
#[derive(Error, Debug)]
pub enum TxDepProviderError {
    #[error("the resource is not found in the provider")]
    NotFound,
    #[error("internal error: `{0}`")]
    Internal(#[from] Box<dyn std::error::Error>),
}

/// Provider dependency information of a transaction:
///   * inputs
///   * cell_deps
///   * header_deps
pub trait TransactionDependencyProvider {
    // For verify certain cell belong to certain transaction
    fn get_tx(&self, tx_hash: H256) -> Result<Transaction, TxDepProviderError>;
    // For get the cell information of inputs or cell_deps
    fn get_cell(&self, out_point: OutPoint) -> Result<CellOutput, TxDepProviderError>;
    // For get the header information of header_deps
    fn get_header(&self, block_hash: H256) -> Result<Header, TxDepProviderError>;
}
