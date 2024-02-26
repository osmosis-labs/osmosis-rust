use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};

#[cw_serde]
pub struct Any {
    pub type_url: String,
    pub value: cosmwasm_std::Binary,
}

#[cw_serde]
pub struct OnAuthenticatorAddedRequest {
    pub account: Addr,
    pub authenticator_id: String,
    pub authenticator_params: Option<Binary>,
}

#[cw_serde]
pub struct OnAuthenticatorRemovedRequest {
    pub account: Addr,
    pub authenticator_id: String,
    pub authenticator_params: Option<Binary>,
}

#[cw_serde]
pub struct AuthenticationRequest {
    pub authenticator_id: String,
    pub account: Addr,
    pub msg: Any,
    pub msg_index: u64,
    pub signature: Binary,
    pub sign_mode_tx_data: SignModeTxData,
    pub tx_data: TxData,
    pub signature_data: SignatureData,
    pub simulate: bool,
    pub authenticator_params: Option<Binary>,
}

#[cw_serde]
pub struct SignModeTxData {
    pub sign_mode_direct: Binary,
    pub sign_mode_textual: Option<String>, // Assuming it's a string or null
}

#[cw_serde]
pub struct TxData {
    pub chain_id: String,
    pub account_number: u64,
    pub sequence: u64,
    pub timeout_height: u64,
    pub msgs: Vec<Any>,
    pub memo: String,
}

#[cw_serde]
pub struct SignatureData {
    pub signers: Vec<Addr>,
    pub signatures: Vec<Binary>,
}

#[cw_serde]
pub struct TrackRequest {
    pub authenticator_id: String,
    pub account: Addr,
    pub msg: Any,
    pub msg_index: u64,
    pub authenticator_params: Option<Binary>,
}

#[cw_serde]
pub struct ConfirmExecutionRequest {
    pub authenticator_id: String,
    pub account: Addr,
    pub msg: Any,
    pub msg_index: u64,
    pub authenticator_params: Option<Binary>,
}
