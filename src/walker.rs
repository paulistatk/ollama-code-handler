use ignore::Walk;
use std::fs::File;

pub fn get_all_content() -> std::io::Result<String> {
    let mut output = String::new();
    for result in Walk::new(".") {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    output.push_str(&format!("Trilha do arquivo: {:?}\n", path));
                    match File::open(&path) {
                        Ok(mut file) => {
                            let mut contents = String::new();
                            if let Err(e) = file.read_to_string(&mut contents) {
                                eprintln!("Erro ao ler arquivo {:?}: {}", path, e);
                            } else {
                                output.push_str(&format!("Conteúdo do arquivo: {}\n", contents));
                            }
                        }
                        Err(e) => eprintln!("Erro ao abrir arquivo {:?}: {}", path, e),
                    }
                }
            }
            Err(err) => println!("ERRO: {}", err),
        }
    }
    Ok(output)
}

use std::io::Read;
use tokio::task;
use crate::run;

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
                            if let Err(e) = run::run(test_output).await {
                                eprintln!("Erro: {}", e);
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
