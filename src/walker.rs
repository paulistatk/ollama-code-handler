use ignore::Walk;
use std::io::{Read, Write};
use tokio::task;
use crate::run;
use std::fs::File;
use serde_json::Value; // Adicione esta linha

pub async fn do_each_file() {
    for result in Walk::new(".") {
        match result {
            Ok(entry) => {
                let path = entry.path().to_path_buf(); // Clone the path
                if path.is_file() {
                    println!("Trilha do arquivo: {:?}", path);
                    let path_clone = path.clone(); // Clone the path again
                    let contents = task::spawn_blocking(move || {
                        let mut file = File::open(&path_clone)?; // Use the cloned path
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)?;
                        Ok::<_, std::io::Error>(contents)
                    })
                    .await
                    .unwrap_or_else(|_| Ok(String::from("Erro ao ler o arquivo")));
                    match contents {
                        Ok(contents) => {
                            let test_output = format!(
                                "Caminho do arquivo: {:?}\nConteúdo do arquivo: {}",
                                path, contents
                            );
                            let result = run::run(test_output).await;
                            match result {
                                Ok(json) => {
                                    // Parse o JSON e obtenha o valor do atributo 'response'
                                    let v: Value = serde_json::from_str(&json).unwrap();
                                    let response = v["response"].as_str().unwrap_or("");

                                    // Escreva o valor do atributo 'response' em um arquivo .md
                                    let output_path = format!("{}.md", path.to_string_lossy());
                                    let mut file = File::create(&output_path).unwrap();
                                    file.write_all(response.as_bytes()).unwrap();
                                }
                                Err(e) => eprintln!("Erro: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Erro ao ler o arquivo: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Erro ao caminhar pelo diretório: {}", e),
        }
    }
}
