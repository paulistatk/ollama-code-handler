use super::http_error::HttpError;
use reqwest::Client;
use std::error::Error;

pub async fn post(endpoint: &str, payload: &str) -> Result<String, Box<dyn Error>> {
    let client: Client = Client::new();
    let res = client
        .post(endpoint)
        .body(payload.to_string())
        .send()
        .await?;

    if res.status().is_success() {
        let body = res.text().await?;
        Ok(body)
    } else {
        Err(Box::new(HttpError {
            status: res.status(),
        }))
    }
}
