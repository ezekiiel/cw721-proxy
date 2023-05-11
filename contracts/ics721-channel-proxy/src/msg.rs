use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub origin: Option<String>,
    pub whitelist: Option<Vec<String>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Owner(String),
    /// Incoming msg from CW721 contract for ICS721 transfer.
    ReceiveNft(cw721::Cw721ReceiveMsg),
    /// Add channel to whitelist.
    AddToWhitelist {
        channel: String,
    },
    /// Remove channel from whitelist.
    RemoveFromWhitelist {
        channel: String,
    },
    Origin(String),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Addr)]
    Owner {},

    /// Gets ICS721 contract.
    #[returns(Addr)]
    Origin {},

    /// Gets a list of channels authorized for ICS721 transfers.
    #[returns(Vec<String>)]
    Whitelist {},

    /// True in case channel is authorized for ICS721 transfers.
    #[returns(bool)]
    WhiteListed { channel: String },
}

#[cw_serde]
pub enum MigrateMsg {
    WithUpdate {
        whitelist: Option<Vec<String>>,
        origin: Option<String>,
    },
}
