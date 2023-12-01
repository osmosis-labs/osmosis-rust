use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};

#[cw_serde]
pub struct Any {
    pub type_url: String,
    pub value: cosmwasm_std::Binary,
}

#[cw_serde]
pub struct AuthenticationRequest {
    pub account: Addr,
    pub msg: Any,
    pub signature: Binary,
    pub sign_mode_tx_data: SignModeTxData,
    pub tx_data: TxData,
    pub signature_data: SignatureData,
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
    pub simulate: bool,
}

#[cw_serde]
pub struct SignatureData {
    pub signers: Vec<Addr>,
    pub signatures: Vec<String>,
}

#[cw_serde]
#[serde(tag = "type", content = "content")]
enum AuthenticationResult {
    Authenticated,
    NotAuthenticated,
    Rejected { msg: String },
}
