mod http_error;
mod fetch;
mod tags;
mod run;

#[tokio::main]
async fn main() {
    if let Err(e) = run::run().await {
        eprintln!("Erro ao executar o programa: {}", e);
    }
}
