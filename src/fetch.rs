use reqwest::Client;
use super::http_error::HttpError;
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use tokio::runtime::Runtime;

    #[test]
    fn test_fetch_success() {
        let _m = mock("GET", "/").with_status(200).create();

        let client = Client::new();
        let mut rt = Runtime::new().unwrap();
        let result = rt.block_on(fetch(&client, &server_url()));

        assert!(result.is_ok());
    }

    #[test]
    fn test_fetch_error() {
        let _m = mock("GET", "/").with_status(500).create();

        let client = Client::new();
        let mut rt = Runtime::new().unwrap();
        let result = rt.block_on(fetch(&client, &server_url()));

        assert!(result.is_err());
    }
}
