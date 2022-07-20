#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub use cosmos_sdk_proto;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const OSMOSISD_VERSION: &str = include_str!("prost/OSMOSIS_COMMIT");

/// Osmosis protobuf definitions.
pub mod epochs {
    pub mod v1beta1 {
        include!("prost/osmosis.epochs.v1beta1.rs");
    }
}

pub mod gamm {
    pub mod v1beta1 {
        include!("prost/osmosis.gamm.v1beta1.rs");
    }

    pub mod poolmodels {
        pub mod balancer {
            pub mod v1beta1 {
                include!("prost/osmosis.gamm.poolmodels.balancer.v1beta1.rs");
            }
        }
        pub mod stableswap {
            pub mod v1beta1 {
                include!("prost/osmosis.gamm.poolmodels.stableswap.v1beta1.rs");
            }
        }
    }
}
pub mod incentives {
    include!("prost/osmosis.incentives.rs");
}

pub mod lockup {
    include!("prost/osmosis.lockup.rs");
}

pub mod mint {
    pub mod v1beta1 {
        include!("prost/osmosis.mint.v1beta1.rs");
    }
}

pub mod poolincentives {
    pub mod v1beta1 {
        include!("prost/osmosis.poolincentives.v1beta1.rs");
    }
}
pub mod superfluid {
    include!("prost/osmosis.superfluid.rs");
    pub mod v1beta1 {
        include!("prost/osmosis.superfluid.v1beta1.rs");
    }
}

pub mod tokenfactory {
    pub mod v1beta1 {
        include!("prost/osmosis.tokenfactory.v1beta1.rs");
    }
}

pub mod txfees {
    pub mod v1beta1 {
        include!("prost/osmosis.txfees.v1beta1.rs");
    }
}
