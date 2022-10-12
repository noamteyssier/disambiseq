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
//!     "ACT".to_string(),
//!     "AGT".to_string()
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
//!     "ACT".to_string(),
//!     "AGT".to_string()
//! ];
//! let dsq = Disambiseq::from_slice(&sequences);
//! 
//! // retrieve a parental sequence
//! assert_eq!(dsq.get_parent("ACT").unwrap().sequence(), "ACT");
//! 
//! // retrieve a mutation sequence's parent
//! assert_eq!(dsq.get_parent("TCT").unwrap().sequence(), "ACT");
//! 
//! // exclude sequences with ambiguous parents
//! assert_eq!(dsq.get_parent("AAT"), None);
//! assert_eq!(dsq.get_parent("ATT"), None);
//! ```

mod disambiseq;
mod sequence;
pub use crate::{
    disambiseq::Disambiseq,
    sequence::Sequence
};
