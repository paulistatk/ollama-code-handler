use super::generate::generate;
use std::error::Error;

pub async fn run(output: String) -> Result<String, Box<dyn Error>> {
    let mut string: String = "converta o seguinte arquivo para php: ".to_string();
    string.push_str(&output);
    // println!("{}", string);
    match generate(&string).await {
        Ok(result) => {
            // println!("Resultado: {}", result);
            return Ok(result);
        }
        Err(e) => {
            eprintln!("Erro ao fazer a requisição: {}", e);
            return Err(e);
        }
    }
}
