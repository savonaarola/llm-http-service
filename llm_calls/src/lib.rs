use std::error::Error;

use tokio::process::Command;

pub async fn query(model: &str, prompt: &str) -> Result<Vec<String>, Box<dyn Error>>{
    let output = Command::new("ollama")
        .arg("run")
        .arg(model)
        .arg(prompt)
        .output()
        .await?;

    if !output.status.success(){
        return Err("ollama run failed".into());
    }

    let lines = String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(lines)
}

pub async fn models() -> Result<Vec<String>, Box<dyn Error>>{
    let output = Command::new("ollama")
        .arg("list")
        .output()
        .await?;

    if !output.status.success(){
        return Err("ollama list  failed".into());
    }

    let lines = String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(lines)
}


#[cfg(test)]
mod tests{
    #[tokio::test]
    async fn models_works(){
        let models = super::models().await.unwrap();
        dbg!(&models);
        assert!(!models.is_empty());
    }
    #[tokio::test]
    async fn query_works(){
        let response = super::query(
            "qwen3:0.6b",
            "Hi, tell me what can you do")
            .await
            .unwrap();
        dbg!(&response);
        assert!(!response.is_empty());
    }
}