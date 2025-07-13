use base64::{Engine, engine::general_purpose};
use hackattic_helper::HackAtticApi;
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2_hmac;
use scrypt::Params;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use thiserror::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = HackAtticApi::new("password_hashing");
    let resp = api.get_challenge().await?;
    dbg!(&resp);
    let body_text: ChallengeResponse = resp.json().await?;
    dbg!(&body_text);
    let sha256_hash = sha256::digest(body_text.password);
    let sha256_bytes = hex::decode(&sha256_hash)?;

    type HmacSha256 = Hmac<Sha256>;

    let mac = HmacSha256::new_from_slice(&sha256_bytes)?;
    let mac_hash = mac.finalize();
    let mut pb_key1 = [0u8; 32];
    let salt = general_purpose::STANDARD.decode(body_text.salt)?;
    dbg!(&salt);

    pbkdf2_hmac::<Sha256>(
        &mac_hash.clone().into_bytes(),
        &salt,
        body_text.pbkdf2.rounds,
        &mut pb_key1,
    );
    dbg!(&pb_key1);
    let scrypt_struct = body_text.scrypt;
    let params = Params::new(
        (scrypt_struct.N as f64).log2() as u8,
        scrypt_struct.r,
        scrypt_struct.p,
        scrypt_struct.buflen,
    )?;
    let mut scrypt_output = vec![0u8; scrypt_struct.buflen];
    let _ = scrypt::scrypt(&pb_key1, &salt, &params, &mut scrypt_output)?;
    let resp = api
        .send_solution(&SolutionPayload {
            sha256: hex::encode(&sha256_bytes),
            hmac: hex::encode(&mac_hash.into_bytes()),
            pbkdf2: hex::encode(&pb_key1),
            scrypt: hex::encode(&scrypt_output),
        })
        .await?;
    dbg!(&resp);
    let challenge_response = resp.text().await?;
    dbg!(challenge_response);
    Ok(())
}

#[derive(Deserialize, Debug)]
struct ChallengeResponse {
    password: String,
    salt: String,
    pbkdf2: Pbkdf2,
    pub scrypt: Scrypt,
}
#[derive(Deserialize, Debug)]

struct Pbkdf2 {
    hash: String,
    rounds: u32,
}
#[derive(Deserialize, Debug)]

struct Scrypt {
    N: u32,
    p: u32,
    r: u32,
    buflen: usize,
    _control: String,
}
#[derive(Serialize, Debug)]
struct SolutionPayload {
    sha256: String,
    hmac: String,
    pbkdf2: String,
    scrypt: String, // ...
}

#[derive(Debug, Error)]
pub enum HackErrors {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
