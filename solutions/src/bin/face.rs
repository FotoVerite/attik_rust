use std::path::Path;

use hackattic_helper::HackAtticApi;
use image::{GenericImageView, ImageBuffer};
use reqwest::Client;
use rustface::{Detector, ImageData};
use thiserror::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = HackAtticApi::new("basic_face_detection");
    let resp = api.get_challenge().await?;
    dbg!(&resp);
    let body_text: ChallengeResponse = resp.json().await?;
    dbg!(&body_text);
    let client = Client::new();
    let resp = client.get(body_text.image_url).send().await?;
    let bytes = resp.bytes().await?;
    let img = image::load_from_memory(&bytes)?.to_luma8();
    let (width, height) = (img.width(), img.height());
    let cols = 8;
    let rows = 8;
    let cell_w = width / cols;
    let cell_h = height / rows;
    let mut attik_respose = vec![];
    let mut detector = load_detector();
    for row in 0..rows {
        for col in 0..cols {
            let x = col * cell_w;
            let y = row * cell_h;
            let cell = img.view(x, y, cell_w, cell_h);
            let raw = cell.to_image().as_raw().clone();
            let img_data = ImageData::new(&raw, cell_w, cell_h);

            let info = detector.detect(&img_data);
            if !info.is_empty() {
                attik_respose.push([row, col]);
            }
        }
    }
    dbg!(&attik_respose);
    let resp = api
        .send_solution(&SolutionPayload {
            face_tiles: attik_respose,
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
    image_url: String,
}

#[derive(Serialize, Debug)]
struct SolutionPayload {
    face_tiles: Vec<[u32; 2]>,
    // ...
}

#[derive(Debug, Error)]
pub enum HackErrors {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

fn load_detector() -> Box<dyn Detector> {
    let mut detector = rustface::create_detector("seed.bin").expect("Failed to parse model");
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);

    detector
}
