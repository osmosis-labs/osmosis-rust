pub mod osmosis {
    pub mod epochs {
        pub mod v1beta1 {
            include!("osmosis.epochs.v1beta1.rs");
        }
    }
    pub mod gamm {
        pub mod poolmodels {
            pub mod balancer {
                pub mod v1beta1 {
                    include!("osmosis.gamm.poolmodels.balancer.v1beta1.rs");
                }
            }
            pub mod stableswap {
                pub mod v1beta1 {
                    include!("osmosis.gamm.poolmodels.stableswap.v1beta1.rs");
                }
            }
        }
        pub mod v1beta1 {
            include!("osmosis.gamm.v1beta1.rs");
        }
    }
    pub mod incentives {
        include!("osmosis.incentives.rs");
    }
    pub mod lockup {
        include!("osmosis.lockup.rs");
    }
    pub mod mint {
        pub mod v1beta1 {
            include!("osmosis.mint.v1beta1.rs");
        }
    }
    pub mod poolincentives {
        pub mod v1beta1 {
            include!("osmosis.poolincentives.v1beta1.rs");
        }
    }
    pub mod store {
        pub mod v1beta1 {
            include!("osmosis.store.v1beta1.rs");
        }
    }
    pub mod superfluid {
        include!("osmosis.superfluid.rs");
        pub mod v1beta1 {
            include!("osmosis.superfluid.v1beta1.rs");
        }
    }
    pub mod tokenfactory {
        pub mod v1beta1 {
            include!("osmosis.tokenfactory.v1beta1.rs");
        }
    }
    pub mod txfees {
        pub mod v1beta1 {
            include!("osmosis.txfees.v1beta1.rs");
        }
    }
}
