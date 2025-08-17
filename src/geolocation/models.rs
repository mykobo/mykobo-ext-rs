use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FreeIpApiResponse {
    pub asn: Option<String>,
    pub asn_organization: Option<String>,
    pub capital: Option<String>,
    pub city_name: Option<String>,
    pub continent: Option<String>,
    pub continent_code: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub currencies: Vec<String>,
    pub ip_address: Option<String>,
    pub ip_version: Option<u8>,
    pub is_proxy: bool,
    pub languages: Vec<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub phone_codes: Vec<u16>,
    pub region_name: Option<String>,
    pub time_zones: Vec<String>,
    pub zip_code: Option<String>,
}
