use httpmock::prelude::*;
use mykobo_ext_rs::geolocation::{GeoLocatorClient, GeoLocatorConfig};
use serde_json::json;

#[tokio::test]
async fn test_geolocator_returns_free_api_response() {
    let free_ip_api_mock_server = MockServer::start();
    let free_ip_data = json!({
        "asn": "29614",
        "asnOrganization": "VODAFONE GHANA AS INTERNATIONAL TRANSIT",
        "capital": "Accra",
        "cityName": "Accra (Kpeshie)",
        "continent": "Africa",
        "continentCode": "AF",
        "countryCode": "GH",
        "countryName": "Ghana",
        "currencies": [
            "GHS"
        ],
        "ipAddress": "102.176.41.169",
        "ipVersion": 4,
        "isProxy": false,
        "languages": [
            "en"
        ],
        "latitude": 5.59966,
        "longitude": -0.176387,
        "phoneCodes": [
            233
        ],
        "regionName": "Greater Accra",
        "timeZones": [
            "Africa/Accra"
        ],
        "zipCode": "-"
    });

    let geolocator_client = GeoLocatorClient::new(GeoLocatorConfig {
        host: format!("http://{}", free_ip_api_mock_server.address()),
        path: "/api/json/".to_string(),
    });

    free_ip_api_mock_server.mock(|when, then| {
        when.method(GET).path("/api/json/102.176.41.169");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!(free_ip_data));
    });

    let response = geolocator_client.geolocate("102.176.41.169").await;
    assert!(response.is_ok());
    let locator_response = response.unwrap();
    assert_eq!(locator_response.country_code, Some("GH".to_string()));
}
