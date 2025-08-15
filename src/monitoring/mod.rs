pub mod models;

use crate::Error;
use crate::monitoring::models::{
    AddressScreeningRequest, AddressScreeningResponse, anchor_to_chain,
};
use reqwest::Client;
use std::env;
use tracing::debug;

pub struct MonitoringResult {
    pub score: u8,
    pub message: String,
    pub address: String,
}

#[allow(dead_code)]
trait Monitoring {
    fn screen(
        &self,
        address: &str,
        anchor: &str,
    ) -> impl Future<Output = Result<MonitoringResult, Error>>;
}

pub struct MerkelScienceMonitoring {
    pub host: String,
    pub api_key: String,
    pub client: Client,
}

impl From<AddressScreeningResponse> for MonitoringResult {
    fn from(response: AddressScreeningResponse) -> Self {
        MonitoringResult {
            score: response.risk_level,
            message: format!("Risk level is {}", response.risk_level_verbose),
            address: response.identifier,
        }
    }
}

impl Default for MerkelScienceMonitoring {
    fn default() -> Self {
        Self::new()
    }
}

impl MerkelScienceMonitoring {
    pub fn new() -> Self {
        let host = env::var("MERKEL_SCIENCE_HOST").expect("MERKEL_SCIENCE_HOST must be set");
        let api_key =
            env::var("MERKEL_SCIENCE_API_KEY").expect("MERKEL_SCIENCE_API_KEY must be set");
        let client = Client::new();
        MerkelScienceMonitoring {
            host,
            api_key,
            client,
        }
    }

    pub async fn screen_address(
        &self,
        address: &str,
        anchor: &str,
    ) -> Result<MonitoringResult, Error> {
        let url = format!("{}/api/v4.2/addresses/", self.host);
        let request = AddressScreeningRequest {
            identifier: address.to_string(),
            blockchain: anchor_to_chain(anchor).to_string(),
        };

        debug!("Sending request to Merkel Science API({}): {:#?}", url, request);

        match self
            .client
            .post(&url)
            .json(&serde_json::json!(request))
            .header("X-API-KEY", self.api_key.clone())
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    let text = response.text().await?;
                    println!("{text}");
                    let result: AddressScreeningResponse = serde_json::from_str(&text)?;
                    Ok(result.into())
                } else {
                    Err(Error::from(response.status()))
                }
            }
            Err(error) => Err(Error::new(format!(
                "Failed to send request to Merkel Science API {error:?}"
            ))),
        }
    }
}

impl Monitoring for MerkelScienceMonitoring {
    async fn screen(&self, address: &str, anchor: &str) -> Result<MonitoringResult, Error> {
        self.screen_address(address, anchor).await
    }
}
