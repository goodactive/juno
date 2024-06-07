pub mod state {
    use candid::CandidType;
    use junobuild_shared::types::core::DomainName;
    use serde::{Deserialize, Serialize};

    #[derive(Default, CandidType, Serialize, Deserialize, Clone)]
    pub struct AuthenticationHeapState {
        pub config: AuthenticationConfig,
    }

    #[derive(Default, CandidType, Serialize, Deserialize, Clone)]
    pub struct AuthenticationConfig {
        pub internet_identity: Option<AuthenticationConfigInternetIdentity>,
    }

    #[derive(Default, CandidType, Serialize, Deserialize, Clone)]
    pub struct AuthenticationConfigInternetIdentity {
        pub derivation_origin: Option<DomainName>,
    }
}
