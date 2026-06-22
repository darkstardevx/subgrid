use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
    http::Method, // Changed from 'use http::Method;'
};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use serde::Deserialize;
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer}; // Required for CORS
use subgrid::App; // Ensure this matches your project name

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
    lang: String,
}

async fn compile_handler(Json(payload): Json<CompileRequest>) -> impl IntoResponse {
    println!("Received request to compile {} code (length: {})", payload.lang, payload.code.len());
    
    Json(serde_json::json!({ 
        "output": format!("Compiler backend received {} bytes of {} code.", payload.code.len(), payload.lang) 
    }))
}

async fn aur_search_handler(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let arg = params.get("arg").map(|s| s.as_str()).unwrap_or("");
    let url = format!("https://aur.archlinux.org/rpc/v5/search/{}", arg);

    match reqwest::get(url).await {
        Ok(resp) => {
            let json = resp.json::<serde_json::Value>().await.unwrap_or_else(|_| serde_json::json!({"error": "Parse failure"}));
            Json(json).into_response()
        }
        Err(_) => Json(serde_json::json!({"error": "AUR unavailable"})).into_response(),
    }
}

#[tokio::main]
async fn main() {
    let conf = get_configuration(Some("Cargo.toml")).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // 1. Configure CORS to fix "TypeError: Failed to fetch"
    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    // 2. Build the router with correct chaining
    let app = Router::new()
        .route("/api/compile", post(compile_handler))
        .route("/api/aur/search", get(aur_search_handler))
        .layer(cors) // Apply CORS here
        .leptos_routes(&leptos_options, routes, App)
        .with_state(leptos_options); // CRITICAL: This was missing in your snippet

    // 3. Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("SUBGRID Server listening on http://0.0.0.0:3000");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
