use std::env;

use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Router {
    Router::new().route("/voice-gen", post(generate_audio))
}

#[derive(Deserialize, Debug)]
struct AudioPayload {
    text: String,
}

async fn generate_audio(Json(payload): Json<AudioPayload>) -> impl IntoResponse {
    let api_secret = env::var("DEEPGRAM_API_SECRET").expect("DEEPGRAM_API_SECRET is not set");

    reqwest::Client::new()
        .post("https://api.deepgram.com/v1/speak?model=aura-arcas-en")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Token {api_secret}"))
        .header("accept", "text/plain")
        .body(json!({ "text": payload.text }).to_string())
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
}
