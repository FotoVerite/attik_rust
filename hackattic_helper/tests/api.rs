use httpmock::prelude::*;
use reqwest;
use tokio;

#[tokio::test]
async fn test_get_challenge_mocked() {
    // Start a local mock server
    let server = MockServer::start();

    // Set up a mock GET response
    let challenge_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/challenges/help_me_unpack/problem")
            .query_param("access_token", "ad511f7fb5136d19");
        then.status(200)
            .header("content-type", "application/json")
            .json_body_obj(&serde_json::json!({
                "example_key": "example_value"
            }));
    });

    // Perform the request against the mock server
    let url = format!(
        "{}/challenges/help_me_unpack/problem?access_token=ad511f7fb5136d19",
        server.base_url()
    );

    let resp = reqwest::get(&url).await.unwrap();
    let json: serde_json::Value = resp.json().await.unwrap();

    assert_eq!(json["example_key"], "example_value");
    challenge_mock.assert(); // verifies the mock was called
}