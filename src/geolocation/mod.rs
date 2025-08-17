pub mod models;

use crate::Error;
use models::FreeIpApiResponse;
use reqwest::{Method, StatusCode};
use tracing::info;

#[derive(Debug, Clone)]
pub struct GeoLocatorConfig {
    pub host: String,
    pub path: String,
}

pub struct GeoLocatorClient {
    geolocator_api_config: GeoLocatorConfig,
}

pub trait Geolocator {
    fn geolocate(
        &self,
        ip: &str,
    ) -> impl std::future::Future<Output = Result<FreeIpApiResponse, Error>>;
}

impl Geolocator for GeoLocatorClient {
    fn geolocate(
        &self,
        ip: &str,
    ) -> impl std::future::Future<Output = Result<FreeIpApiResponse, Error>> {
        self.geolocate(ip)
    }
}

impl GeoLocatorClient {
    pub fn new(geolocator_api_config: GeoLocatorConfig) -> Self {
        GeoLocatorClient {
            geolocator_api_config,
        }
    }

    pub async fn geolocate(&self, ip: &str) -> Result<FreeIpApiResponse, Error> {
        Self::find(ip, self.geolocator_api_config.clone()).await
    }

    pub async fn find(ip: &str, config: GeoLocatorConfig) -> Result<FreeIpApiResponse, Error> {
        let url = format!("{}{}{ip}", config.host, config.path);
        info!("Geolocating IP with {url}");
        let client = reqwest::Client::new();
        match client
            .request(Method::GET, url)
            .header("Accept", "Application/Json")
            .send()
            .await
        {
            Ok(response) if response.status() == StatusCode::OK => {
                response.json().await.map_err(Error::from)
            }
            Ok(response) => {
                let text = response.text().await?;
                println!("GEOLOCATOR RESPONSE: {text}");
                let error: Error = serde_json::from_str(&text)?;
                Err(error)
            }
            Err(e) => Err(Error::from(e)),
        }
    }
}
