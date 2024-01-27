use super::http_error::HttpError;
use reqwest::Client;
use std::error::Error;

pub async fn fetch(client: &Client, endpoint: &str) -> Result<String, Box<dyn Error>> {
    let res = client.get(endpoint).send().await?;

    if res.status().is_success() {
        let body = res.text().await?;
        Ok(body)
    } else {
        Err(Box::new(HttpError {
            status: res.status(),
        }))
    }
}
