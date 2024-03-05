use crate::helpers::spawn_app;

#[tokio::test]
async fn srs_api_handler_returns_valid_data_for_get_info_request() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();


#[tokio::test]
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
