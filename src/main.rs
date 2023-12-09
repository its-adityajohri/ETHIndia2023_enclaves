use tokio::task;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser_handle_1 = task::spawn(async {
        let _ = parser::parse(44787, 44787).await;
    });

    parser_handle_1.await?;
    Ok(())
}
