use super::fetch::fetch;
use reqwest::Client;
use std::error::Error;

pub async fn tags() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let endpoint = "http://localhost:11434/api/tags";
    let result = fetch(&client, endpoint).await?;
    Ok(result)
}
