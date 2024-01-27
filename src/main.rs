mod fetch;
mod generate;
mod http_error;
mod post;
mod run;
mod tags;

#[tokio::main]
async fn main() {
    if let Err(e) = run::run().await {
        eprintln!("Erro ao executar o programa: {}", e);
    }
}
