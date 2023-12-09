use tokio::task;
mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser_handle_1 = task::spawn(async {
        let _ = parser::parse(44787).await;
    });

    let parser_handle_2 = task::spawn(async {
        let _ = parser::parse(11155111).await;
    });

    parser_handle_1.await?;
    parser_handle_2.await?;

    Ok(())
}
