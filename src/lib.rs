//! # Orca Public API Client
//!
//! A Rust client library for the Orca Public API.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use orca_public_api_client::client::client::OrcaClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = OrcaClient::new();
//!     let protocol_info = client.get_protocol_info("solana").await.unwrap();
//!     println!("{:?}", protocol_info);
//! }
//! ```

pub mod client;
pub mod models;
