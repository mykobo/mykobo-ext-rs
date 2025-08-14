use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct TagDetail {
    pub tag_type_verbose: String,
    pub tag_subtype_verbose: String,
    pub tag_name_verbose: String,
}

#[derive(Debug, Deserialize)]
pub struct TagExposure {
    pub tag_type_verbose: String,
    pub tag_subtype_verbose: String,
    pub tag_name_verbose: String,
    pub total_value_usd: f64,
    pub exposure_type: String,
}

#[derive(Debug, Deserialize)]
pub struct DigitalAsset {
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    pub owner: TagDetail,
    pub user: TagDetail,
}

#[derive(Debug, Deserialize)]
pub struct AddressScreeningResponse {
    pub identifier: String,
    pub blockchain: u8,
    pub blockchain_verbose: String,
    pub r#type: Option<String>,
    pub type_verbose: Option<String>,
    pub total_incoming_value: String,
    pub total_incoming_value_usd: String,
    pub total_outgoing_value: String,
    pub total_outgoing_value_usd: String,
    pub balance: i32,
    pub earliest_transaction_time: String,
    pub latest_transaction_time: String,
    pub risk_level: u8,
    pub risk_level_verbose: String,
    pub created_at: String,
    pub updated_at: String,
    pub workspace: Workspace,
    pub originator: Vec<TagExposure>,
    pub beneficiary: Vec<TagExposure>,
    pub tags: Tags,
    pub digital_assets: Vec<DigitalAsset>,
    pub custom_tags: Vec<String>,
    pub is_megahub: bool,
    pub customer_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AddressScreeningRequest {
    pub identifier: String,
    pub block_chain: u8,
}

pub fn anchor_to_chain(anchor: &str) -> u8 {
    match anchor.to_uppercase().as_str() {
        "ANCHOR_PLATFORM" => 13,
        _ => 0, // Default or unknown blockchain
    }
}
