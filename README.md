# Ollama HTTP Service

HTTP interface for Ollama LLMs, built with Rust's async ecosystem.

## Features
- Query Ollama models via HTTP
- List available models
- Minimal overhead

## Technologies
- Rust
- Axum (HTTP server)
- Tokio (async runtime)
- Serde (JSON serialization)

## Prerequisites
- [Ollama](https://ollama.com/) installed 
- Rust toolchain

## Installation

git clone https://github.com/savonaarola/llm-http-service.git
cd llm-http-service
cargo run -p llm_http

# List models
curl http://localhost:3000/models

# Query a model
curl -X POST http://localhost:3000/query \
  -H "Content-Type: application/json" \
  -d '{"model": "llama2", "prompt": "Explain Rust borrow checker"}'