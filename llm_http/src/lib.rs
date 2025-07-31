use std::error::Error;
use axum::{routing,Router, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
pub async fn run_llm_http() -> Result<(), Box<dyn Error>>{
    let app = Router::new()
    .route("/models", routing::get(get_models))
    .route("/query", routing::get(process_query));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_models() -> Result<Json<Vec<String>>, StatusCode> {
    llm_calls::models().await.map(Json).map_err(|e|{
        eprintln!("Failed to get models: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn process_query(Json(params): Json<LlmQuery>) -> Result<Json<Vec<String>>, StatusCode>{
    llm_calls::query(&params.model,&params.prompt).await.map(Json).map_err(|e|{
        eprintln!("Failed to process LLM query: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(Debug, Serialize,Deserialize)]
struct LlmQuery{
    pub model: String,
    pub prompt: String,
}