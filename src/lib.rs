//! # disambiseq
//!
//! A Rust library for creating DNA sequence lookup tables with single-mismatch tolerance.
//!
//! ## Summary
//!
//! Creates hash tables that can match sequences even with single-base errors.
//! Automatically removes ambiguous sequences that could match multiple parents.
//!
//! ## Usage
//!
//! ```rust
//! use disambiseq::{Disambiseq, Error};
//!
//! fn main() -> Result<(), Error> {
//!     // Create lookup table from parent sequences
//!     let library = vec![
//!         b"ACTG".to_vec(),
//!         b"AGTC".to_vec(),
//!         b"ACTC".to_vec(),
//!     ];
//!     let disambiseq = Disambiseq::new(&library)?;
//!
//!     // Find matches (exact or single mismatch)
//!     assert_eq!(disambiseq.get_index(b"ACTG"), Some(0));  // Exact match
//!     assert_eq!(disambiseq.get_index(b"CCTG"), Some(0));  // 1 mismatch
//!     assert_eq!(disambiseq.get_index(b"AGTG"), None);     // Ambiguous match
//!     assert_eq!(disambiseq.get_index(b"TTTT"), None);     // No match
//!
//!     // Get original parent sequence
//!     let parent = disambiseq.get_parent(b"CCTG");
//!     assert_eq!(parent, Some(b"ACTG".as_slice()));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## API
//!
//! - **`Disambiseq::new(sequences)`** - Create lookup with mismatch tolerance
//! - **`Disambiseq::new_exact(sequences)`** - Create lookup for exact matches only
//! - **`get_index(sequence)`** - Find parent index for a sequence
//! - **`get_parent(sequence)`** - Get parent sequence for a sequence
//! - **`index_parent(index)`** - Get parent sequence for an index
//!
//! ## Use cases
//!
//! - Barcode demultiplexing with sequencing errors
//! - Primer matching with mismatches
//! - Sequence classification with error tolerance

mod error;
mod map;

pub use error::Error;
pub use map::Disambiseq;
