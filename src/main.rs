mod parser;

use std::thread;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser_handle_1 = thread::spawn({
        move || {
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                let _ = parser::parse(1, 2).await;
            });
        }
    });

    parser_handle_1.join().unwrap();
    Ok(())
}
