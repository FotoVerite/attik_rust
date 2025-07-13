use base64::{Engine, engine::general_purpose};
use hackattic_helper::HackAtticApi;
use thiserror::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = HackAtticApi::new("dockerized_solutions");
    let resp = api.get_challenge().await?;
    dbg!(&resp);
    let body_text: ChallengeResponse = resp.json().await?;
    dbg!(&body_text);
    
    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ChallengeResponse {
    credentials: Credentials,
    ignition_key: String, 
    trigger_token: String
}
#[derive(Deserialize, Debug)]
struct Credentials {
    user: String,
    password: String, 
}

#[derive(Serialize, Debug)]
struct SolutionPayload {
    certificate: String,
    // ...
}

#[derive(Debug, Error)]
pub enum HackErrors {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
