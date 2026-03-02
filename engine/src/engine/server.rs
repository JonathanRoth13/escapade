use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::analyze::analyze;
use super::parsing::parse_node;
use crate::core::{DEPTH_0_SENTINEL, validate_node};
use crate::tablebase::TablebaseIndex;

type SharedState = Arc<Option<TablebaseIndex>>;

#[derive(Deserialize)]
struct AnalyzeRequest {
    node: String,
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn version() -> Json<Value> {
    Json(json!({ "version": env!("CARGO_PKG_VERSION") }))
}

async fn analyze_handler(
    State(tablebase): State<SharedState>,
    Json(body): Json<AnalyzeRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let node = if body.node == "root" {
        DEPTH_0_SENTINEL
    } else {
        let parsed = parse_node(&body.node).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!([e.to_string()])),
            )
        })?;

        validate_node(&parsed).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!([e.to_string()])),
            )
        })?;

        parsed
    };

    let json_string = analyze(&node, tablebase.as_ref().as_ref());
    let value: Value = serde_json::from_str(&json_string).unwrap_or(json!({}));
    Ok(Json(value))
}

pub async fn run(tablebase: Option<TablebaseIndex>, listen: &str) {
    println!("escapade {}", env!("CARGO_PKG_VERSION"));

    if let Some(tb) = &tablebase {
        let available = tb.available_depths();
        let loaded_depths: Vec<usize> = available
            .iter()
            .enumerate()
            .filter_map(|(idx, &loaded)| if loaded { Some(idx) } else { None })
            .collect();
        println!(
            "tablebase loaded: depths={:?}, memory={}",
            loaded_depths,
            tb.memory_usage()
        );
    } else {
        println!("tablebase not loaded");
    }

    let state: SharedState = Arc::new(tablebase);

    let app = Router::new()
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/analyze", post(analyze_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(listen)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to bind to {}: {}", listen, e);
            std::process::exit(1);
        });

    println!("listening on {}", listen);

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Server error: {}", e);
            std::process::exit(1);
        });
}
