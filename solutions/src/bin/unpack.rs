use anyhow::Context;
use base64::{Engine, engine::general_purpose};
use hackattic_helper::HackAtticApi;
use thiserror::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = HackAtticApi::new("help_me_unpack");
    let resp = api.get_challenge().await?;
    dbg!(&resp);
    let body_text: ChallengeResponse = resp.json().await?;
    dbg!(&body_text);
    let decoded_bytes = general_purpose::STANDARD
        .decode(&body_text.bytes.as_bytes())
        .context("issue with parsing")?;
    let bytes: &[u8] = &decoded_bytes;

    dbg!(&decoded_bytes);
    let signed_int = i32::from_le_bytes(bytes[0..4].try_into()?);
    let unsigned_int = u32::from_le_bytes(bytes[4..8].try_into()?);
    let short = i16::from_le_bytes(bytes[8..10].try_into()?);
    let float = f32::from_le_bytes(bytes[12..16].try_into()?);
    let double = f64::from_le_bytes(bytes[16..24].try_into()?);
    let big_endian_double = f64::from_be_bytes(bytes[24..32].try_into()?);
    let resp = api
        .send_solution(&SolutionPayload {
            int: signed_int,
            uint: unsigned_int,
            short,
            float,
            double,
            big_endian_double,
        })
        .await?;
    dbg!(&resp);
    let challenge_response = resp.text().await?;
    dbg!(challenge_response);
    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ChallengeResponse {
    bytes: String,
    // ...
}

#[derive(Serialize, Debug)]
struct SolutionPayload {
    int: i32,
    uint: u32,
    short: i16,
    float: f32,
    double: f64,
    big_endian_double: f64,
    // ...
}

#[derive(Debug, Error)]
pub enum HackErrors {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
