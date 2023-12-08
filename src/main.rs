mod parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let _ = parser::parse().await;

    Ok(())
}
