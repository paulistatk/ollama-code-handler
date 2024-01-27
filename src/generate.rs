use super::post::post;
use serde_json::json;
use std::error::Error;

pub async fn generate(payload: &String) -> Result<String, Box<dyn Error>> {
    // Cria um objeto JSON
    let json = json!({
        "model": "mistral",
        "prompt": payload,
        "stream": false
    });

    // Converte o objeto JSON em uma string
    let json_string = json.to_string();
    let endpoint = "http://localhost:11434/api/generate";
    let result = post(endpoint, &json_string).await?;
    Ok(result)
}
