use mykobo_ext_rs::Error;
use mykobo_ext_rs::geolocation::models::FreeIpApiResponse;
use pretty_assertions::assert_eq;

#[test]
fn test_empty_free_ip_api_response_deserialisation() {
    let json_data = r#"
    {
        "asn": null,
        "asnOrganization": null,
        "capital": null,
        "cityName": null,
        "continent": null,
        "continentCode": null,
        "countryCode": null,
        "countryName": null,
        "currencies": [],
        "ipAddress": null,
        "ipVersion": null,
        "isProxy": true,
        "languages": [],
        "latitude": null,
        "longitude": null,
        "phoneCodes": [],
        "regionName": null,
        "timeZones": [],
        "zipCode": "412415"
    }"#;

    let deserialized = serde_json::from_str::<FreeIpApiResponse>(json_data);
    assert!(deserialized.is_ok());
    let response = deserialized.unwrap();
    assert_eq!(response.zip_code, Some("412415".to_string()))
}

#[test]
fn test_free_ip_api_response_deserialisation() {
    let json_data = r#"
    {
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
    }"#;

    let deserialized = serde_json::from_str::<FreeIpApiResponse>(json_data);
    assert!(deserialized.is_ok());
    let response = deserialized.unwrap();
    assert!(!response.is_proxy);
    assert_eq!(response.country_code, Some("GH".to_string()));
    assert_eq!(response.zip_code, Some("-".to_string()));
}

#[test]
fn test_error_deserialisation() {
    let json_data = r#"
    {
        "status":"fail",
        "message":"SSL unavailable for this endpoint, order a key at https://members.ip-api.com/"
    }"#;

    let deserialized = serde_json::from_str::<Error>(json_data);
    assert!(deserialized.is_ok());

    let error = deserialized.unwrap();
    assert_eq!(
        error.message,
        "SSL unavailable for this endpoint, order a key at https://members.ip-api.com/".to_string()
    );

    assert_eq!(
        "SSL unavailable for this endpoint, order a key at https://members.ip-api.com/",
        error.to_string()
    );
}

#[test]
fn test_error_deserialisation_ie() {
    let json_data = r#"
   {
      "ipVersion": 4,
      "ipAddress": "50.7.4.149",
      "latitude": 53.3183,
      "longitude": -6.4404,
      "countryName": "Ireland",
      "countryCode": "IE",
      "capital": "Dublin",
      "phoneCodes": [
        353
      ],
      "timeZones": [
        "Europe/Dublin"
      ],
      "zipCode": "22",
      "cityName": "Saggart (Kilcarbery)",
      "regionName": "Leinster",
      "continent": "Europe",
      "continentCode": "EU",
      "currencies": [
        "EUR"
      ],
      "languages": [
        "ga",
        "en"
      ],
      "asn": "30058",
      "asnOrganization": "FDCservers.net",
      "isProxy": false
    }
   "#;

    let deserialized = serde_json::from_str::<FreeIpApiResponse>(json_data);
    assert!(deserialized.is_ok());
}
