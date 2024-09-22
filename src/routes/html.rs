use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::json;
use std::{fs, io::Write};
use tracing_subscriber::fmt::MakeWriter;
use uuid::Uuid;

#[derive(Deserialize)]
struct HtmlPayload {
    html: String,
}

pub fn routes() -> Router {
    Router::new().route("/html-gen", post(generate_html))
}

async fn generate_html(Json(payload): Json<HtmlPayload>) -> impl IntoResponse {
    let file_name = Uuid::new_v4();
    let html = fs::File::create(format!("/home/html/{}.html", file_name)).unwrap();

    write!(html.make_writer(), "{}", payload.html).unwrap();

    Json(json!({ "file": format!("{file_name}.html") }))
}
