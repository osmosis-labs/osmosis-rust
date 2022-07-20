use cosmwasm_std::{CosmosMsg, Empty};

// pub trait ToCosmosMsg: ::prost::Message + Sized {
//     const TYPE_URL: &'static str;

//     fn to_cosmos_msg(&self) -> Result<CosmosMsg, cosmwasm_std::StdError> {
//         let mut bytes = Vec::new();
//         prost::Message::encode(self, &mut bytes).map_err(|e| {
//             cosmwasm_std::StdError::SerializeErr {
//                 source_type: Self::TYPE_URL.to_string(),
//                 msg: format!("{:?}", e),
//             }
//         })?;
//         Ok(CosmosMsg::<Empty>::Stargate {
//             type_url: Self::TYPE_URL.to_string(),
//             value: cosmwasm_std::Binary(bytes),
//         })
//     }
// }
