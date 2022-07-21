#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub use cosmos_sdk_proto;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const OSMOSISD_VERSION: &str = include_str!("types/OSMOSIS_COMMIT");

pub mod types;
