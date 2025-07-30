//! # disambiseq
//!
//! ## Background
//!
//! I've rewritten this functionality a few times for different use cases
//! and put it into a standalone crate since it might be useful to others.
//!
//! This is used to generate unambiguous one-off mismatch libraries for
//! a set of DNA sequences.
//!
//! ## Usage
//!
//! ### Creating a new unambiguous set
//!
//! ```rust
//! use disambiseq::Disambiseq;
//!
//! let sequences = vec![
//!     b"ACT".to_vec(),
//!     b"AGT".to_vec()
//! ];
//! let dsq = Disambiseq::from_slice(&sequences);
//! println!("{:#?}", dsq);
//! ```
//!
//! ### Visualizing the set
//!
//! ```text
//! Disambiseq {
//!     unambiguous: {
//!         "TCT": "ACT",
//!         "ACA": "ACT",
//!         "CCT": "ACT",
//!         "ACC": "ACT",
//!         "CGT": "AGT",
//!         "GGT": "AGT",
//!         "AGA": "AGT",
//!         "GCT": "ACT",
//!         "ACG": "ACT",
//!         "TGT": "AGT",
//!         "AGC": "AGT",
//!         "AGT": "ACT",
//!         "AGG": "AGT",
//!     },
//!     parents: {
//!         "AGT",
//!         "ACT",
//!     },
//!     ambiguous: {
//!         "ATT",
//!         "AAT",
//!     },
//! }
//! ```
//!
//! ### Querying the Set
//!
//! ```rust
//! use disambiseq::Disambiseq;
//!
//! let sequences = vec![
//!     b"ACT".to_vec(),
//!     b"AGT".to_vec()
//! ];
//! let dsq = Disambiseq::from_slice(&sequences);
//!
//! // retrieve a parental sequence
//! assert_eq!(dsq.get_parent(b"ACT").unwrap().sequence(), b"ACT");
//!
//! // retrieve a mutation sequence's parent
//! assert_eq!(dsq.get_parent(b"TCT").unwrap().sequence(), b"ACT");
//!
//! // exclude sequences with ambiguous parents
//! assert_eq!(dsq.get_parent(b"AAT"), None);
//! assert_eq!(dsq.get_parent(b"ATT"), None);
//! ```

mod map;
mod sequence;
mod utils;
pub use crate::{
    map::{ByteWrapper, Disambiseq},
    sequence::Sequence,
};
