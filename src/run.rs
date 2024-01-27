use super::generate::generate;

use super::tags::tags;
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    match tags().await {
        Ok(result) => {
            println!("Resultado: {}", result);
            let string = "ma oe".to_string();
            match generate(&string).await {
                Ok(result) => {
                    println!("Resultado: {}", result);
                    return Ok(());
                }
                Err(e) => {
                    eprintln!("Erro ao fazer a requisição: {}", e);
                    return Err(e);
                }
            }
        }
        Err(e) => {
            eprintln!("Erro ao fazer a requisição: {}", e);
            return Err(e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_run_success() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(run());

        assert!(result.is_ok());
    }
}
