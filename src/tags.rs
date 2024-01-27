use reqwest::Client;
use std::error::Error;
use super::fetch::fetch;

pub async fn tags() -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let endpoint = "http://localhost:11434/api/tags";
    let result = fetch(&client, endpoint).await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use tokio::runtime::Runtime;

    #[test]
    fn test_tags_success() {
        let mut rt = Runtime::new().unwrap();
        let result = rt.block_on(tags());

        assert!(result.is_ok());
    }
}
