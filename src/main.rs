mod fetch;
mod generate;
mod http_error;
mod post;
mod run;
mod tags;
mod walker;

#[tokio::main]
async fn main() {
    // Chamar get_all_content
    // match walker::get_all_content() {
    //     Ok(output) => {
    //         if let Err(e) = run::run(output).await {
    //             eprintln!("Erro: {}", e);
    //         }
    //     }
    //     Err(e) => eprintln!("Erro: {}", e),
    // }

    // Chamar do_each_file
    walker::do_each_file().await;
}
