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
    pub simulate: bool,
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
    pub signatures: Vec<String>,
}

#[cw_serde]
pub struct TrackRequest {
    pub account: Addr,
    pub msg: Any,
}

#[cw_serde]
pub struct ConfirmExecutionRequest {
    pub account: Addr,
    pub msg: Any,
}

#[cw_serde]
#[serde(tag = "type", content = "content")]
pub enum AuthenticationResult {
    Authenticated,
    NotAuthenticated,
    Rejected { msg: String },
}

impl Into<cosmwasm_std::Binary> for AuthenticationResult {
    fn into(self) -> cosmwasm_std::Binary {
        cosmwasm_std::Binary::from(
            serde_json_wasm::to_string(&self)
                .expect("Failed to serialize AuthenticationResult")
                .as_bytes()
                .to_vec(),
        )
    }
}

#[cw_serde]
#[serde(tag = "type", content = "content")]
pub enum ConfirmationResult {
    Confirm,
    Block { msg: String },
}

impl Into<cosmwasm_std::Binary> for ConfirmationResult {
    fn into(self) -> cosmwasm_std::Binary {
        cosmwasm_std::Binary::from(
            serde_json_wasm::to_string(&self)
                .expect("Failed to serialize ConfirmationResult")
                .as_bytes()
                .to_vec(),
        )
    }
}
