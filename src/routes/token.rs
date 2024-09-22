use std::env;

use axum::{response::IntoResponse, routing::get, Json, Router};
use livekit_api::access_token;
use serde_json::json;

pub fn routes() -> Router {
    Router::new().route("/token-gen", get(create_token_route))
}

fn create_token() -> String {
    let api_key = env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set");
    let api_secret = env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set");

    let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_identity("identity")
        .with_name("name")
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: "my-room".to_string(),
            ..Default::default()
        })
        .to_jwt();

    return token.unwrap();
}

async fn create_token_route() -> impl IntoResponse {
    let response = json!({ "token": format!("{}", create_token()) });

    Json(response)
}
