use signum_node_rs::srs_api::request_models::GetInfoRequestModel;

use crate::helpers::spawn_app;

#[tokio::test]
async fn srs_api_handler_returns_valid_data_for_get_info_request() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "requestType": "getInfo",
        "application": "SignumRust",
        "version": "0.1.0",
        "platform": "Test",
        "shareAddress": false,
        "networkName": "Signum-TEST",
    });
    println!("SENDING: {:#?}", &body);
    // Act
    let response = client
        .post(&format!("{}/", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_ne!(Some(0), response.content_length());
    let json = response.json::<GetInfoRequestModel>().await;
    //TODO: Remove this and do an actual test on test data, probably input from TestApp
    println!("JSON IS: {:#?}", json);
}

#[tokio::test]
#[ignore]
async fn srs_api_handler_returns_valid_data_for_add_peers_request() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "requestType": "getInfo",
    });
    // Act
    let response = client
        .post(&format!("{}/", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
#[ignore]
async fn srs_api_handler_returns_valid_data_for_get_peers_request() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "requestType": "getPeers",
    });

    // Act
    let response = client
        .post(&format!("{}/", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_ne!(Some(0), response.content_length());
}
