use std::error::Error;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    llm_http::run_llm_http().await
}