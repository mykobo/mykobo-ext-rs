use mykobo_ext_rs::monitoring::models::AddressScreeningResponse;

#[test]
fn test_address_screening_deserialisation() {
    let risky_payload = r#"
    {
      "identifier": "GA223OFHVKVAH2NBXP4AURJRVJTSOVHGBMKJNL6GRJWNN4SARVGSITYG",
      "blockchain": 13,
      "blockchain_verbose": "Stellar",
      "type": null,
      "type_verbose": null,
      "total_incoming_value": "16.0000",
      "total_incoming_value_usd": "1.46",
      "total_outgoing_value": "11.1160",
      "total_outgoing_value_usd": "3.44",
      "balance": 0,
      "earliest_transaction_time": "2023-02-01T15:10:57Z",
      "latest_transaction_time": "2025-06-13T05:39:23Z",
      "risk_level": 5,
      "risk_level_verbose": "Critical",
      "created_at": "2025-08-14T08:53:33.346501Z",
      "updated_at": "2025-08-14T08:53:33.530132Z",
      "workspace": {
        "name": "Mykobo",
        "slug": "mykobo"
      },
      "originator": [
        {
          "tag_type_verbose": "Scam",
          "tag_subtype_verbose": "Fake service",
          "tag_name_verbose": "Reported Scam Stellar.expert",
          "total_value_usd": 1.46,
          "exposure_type": "direct"
        }
      ],
      "beneficiary": [
        {
          "tag_type_verbose": "Scam",
          "tag_subtype_verbose": "Fake service",
          "tag_name_verbose": "Reported Scam Stellar.expert",
          "total_value_usd": 0.24,
          "exposure_type": "indirect"
        }
      ],
      "tags": {
        "owner": {
          "tag_type_verbose": "Scam",
          "tag_subtype_verbose": "Fake service",
          "tag_name_verbose": "Reported Scam Stellar.expert"
        },
        "user": {
          "tag_type_verbose": "Scam",
          "tag_subtype_verbose": "Fake service",
          "tag_name_verbose": "Reported Scam Stellar.expert"
        }
      },
      "digital_assets": [
        {
          "name": "Stellar",
          "symbol": "XLM"
        }
      ],
      "custom_tags": [],
      "is_megahub": false,
      "customer_id": null
    }
    "#;

    let trusty_payload = r#"
    {
      "identifier": "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6",
      "blockchain": 13,
      "blockchain_verbose": "Stellar",
      "type": null,
      "type_verbose": null,
      "total_incoming_value": "0.0000",
      "total_incoming_value_usd": "0.00",
      "total_outgoing_value": "0.0000",
      "total_outgoing_value_usd": "0.00",
      "balance": 0,
      "earliest_transaction_time": null,
      "latest_transaction_time": null,
      "risk_level": 0,
      "risk_level_verbose": "No Risk Detected",
      "created_at": "2025-08-15T16:47:41.307661Z",
      "updated_at": "2025-08-15T16:47:41.307674Z",
      "workspace": {
        "name": "Mykobo",
        "slug": "mykobo"
      },
      "originator": [],
      "beneficiary": [],
      "tags": {
        "owner": {},
        "user": {}
      },
      "digital_assets": [],
      "custom_tags": [],
      "is_megahub": false,
      "customer_id": null
    }
    "#;

    let response: AddressScreeningResponse = serde_json::from_str(risky_payload).unwrap();
    assert_eq!(response.risk_level, 5);
    assert_eq!(response.risk_level_verbose, "Critical");
    assert_eq!(
        response.identifier,
        "GA223OFHVKVAH2NBXP4AURJRVJTSOVHGBMKJNL6GRJWNN4SARVGSITYG"
    );

    let ok_response: AddressScreeningResponse = serde_json::from_str(trusty_payload).unwrap();
    assert_eq!(ok_response.risk_level, 0);
    assert_eq!(ok_response.risk_level_verbose, "No Risk Detected");
    assert_eq!(
        ok_response.identifier,
        "GCGRZQ2OZWQVUWSRAFXSNL3N2KF4IVDOONNFBRP2G3622JJYCUYBCQE6"
    );
}

#[test]
fn test_solana_deserialisation() {
    let payload = r#"
    {
      "identifier": "B2JAtKctzWLt4cegWpqBjRqABZDxSSBCNCXPP7Kyk24J",
      "blockchain": 24,
      "blockchain_verbose": "Solana",
      "type": null,
      "type_verbose": null,
      "total_incoming_value": "0.0000",
      "total_incoming_value_usd": "0.00",
      "total_outgoing_value": "0.0000",
      "total_outgoing_value_usd": "0.00",
      "balance": 0,
      "earliest_transaction_time": null,
      "latest_transaction_time": null,
      "risk_level": 0,
      "risk_level_verbose": "No Risk Detected",
      "created_at": "2025-11-21T17:50:39.693886Z",
      "updated_at": "2025-11-21T17:50:39.693900Z",
      "workspace": {
        "name": "Mykobo",
        "slug": "mykobo"
      },
      "originator": [],
      "beneficiary": [],
      "tags": {
        "owner": {},
        "user": {}
      },
      "digital_assets": [],
      "custom_tags": [],
      "is_megahub": false,
      "customer_id": null
    }
    "#;

    let ok_response: AddressScreeningResponse = serde_json::from_str(payload).unwrap();
    assert_eq!(ok_response.risk_level, 0);
    assert_eq!(ok_response.risk_level_verbose, "No Risk Detected");
    assert_eq!(ok_response.identifier, "B2JAtKctzWLt4cegWpqBjRqABZDxSSBCNCXPP7Kyk24J".to_string())
}
