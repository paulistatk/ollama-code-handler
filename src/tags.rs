use super::fetch::fetch;
use std::error::Error;

pub async fn tags() -> Result<String, Box<dyn Error>> {
    let endpoint = "http://localhost:11434/api/tags";
    let result = fetch(endpoint).await?;
    Ok(result)
}
