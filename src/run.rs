use std::error::Error;
use super::tags::tags;

pub async fn run() -> Result<(), Box<dyn Error>> {
    match tags().await {
        Ok(result) => {
            println!("Resultado: {}", result);
            Ok(())
        }
        Err(e) => {
            eprintln!("Erro ao fazer a requisição: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use tokio::runtime::Runtime;

    #[test]
    fn test_run_success() {
        let mut rt = Runtime::new().unwrap();
        let result = rt.block_on(run());

        assert!(result.is_ok());
    }
}
