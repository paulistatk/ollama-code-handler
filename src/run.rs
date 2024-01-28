use super::generate::generate;
use std::error::Error;

pub async fn run(output: String) -> Result<(), Box<dyn Error>> {
    let mut string: String = "gere testes de unidade para o seguinte projeto: ".to_string();
    string.push_str(&output);
    println!("{}", string);
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
