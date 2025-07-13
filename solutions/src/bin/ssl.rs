use anyhow::Context;
use base64::{Engine, engine::general_purpose};
use hackattic_helper::HackAtticApi;
use openssl::{
    asn1::{Asn1Integer, Asn1IntegerRef, Asn1Time, Asn1Type},
    bn::BigNum,
    hash::MessageDigest,
    pkcs7::Pkcs7,
    pkey::PKey,
    pkey_ctx::{PkeyCtx, PkeyCtxRef},
    rsa::Rsa,
    ssl::NameType,
    x509::{X509Builder, X509NameBuilder},
};
use std::io::Read;
use thiserror::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = HackAtticApi::new("tales_of_ssl");
    let resp = api.get_challenge().await?;
    dbg!(&resp);
    let body_text: ChallengeResponse = resp.json().await?;
    dbg!(&body_text);
    let der = general_purpose::STANDARD.decode(body_text.private_key.as_bytes())?;
    let rsa = Rsa::private_key_from_der(&der)?;
    let private_key = PKey::from_rsa(rsa)?;
    let mut builder = X509Builder::new()?;
    let mut name_builder = X509NameBuilder::new()?;
    let serial_number_raw = body_text.required_data.serial_number.replace("0x", "");
    let mut asn1_integer =
        Asn1Integer::from_bn(BigNum::from_hex_str(&serial_number_raw)?.as_ref())?;
    builder.set_serial_number(&asn1_integer)?;
    name_builder.append_entry_by_text_with_type(
        "CN",
        &body_text.required_data.domain,
        Asn1Type::IA5STRING,
    )?;
    name_builder.append_entry_by_text_with_type(
        "C",
        "CX",
        Asn1Type::PRINTABLESTRING,
    )?;
    name_builder.append_entry_by_text_with_type(
        "serialNumber",
        &serial_number_raw,
        Asn1Type::PRINTABLESTRING,
    )?;
    let name = name_builder.build();
    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;
    builder.set_pubkey(&private_key)?;
    builder.set_not_before(&Asn1Time::days_from_now(0).unwrap())?;

    builder.set_not_after(&Asn1Time::days_from_now(365).unwrap())?;
    builder.sign(&private_key, MessageDigest::sha256())?;
    let cert = builder.build();
    let response_der = cert.to_der()?;
    let certificate = general_purpose::STANDARD.encode(response_der);
    dbg!(&certificate);
    let resp = api.send_solution(&SolutionPayload { certificate }).await?;
    dbg!(&resp);
    let challenge_response = resp.text().await?;
    dbg!(challenge_response);
    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ChallengeResponse {
    private_key: String,
    required_data: RequiredData,
}
#[derive(Deserialize, Debug)]
struct RequiredData {
    domain: String,
    serial_number: String,
    country: String,
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
