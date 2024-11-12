mod server;
mod model;

use std::error::Error;
use dotenv::dotenv;
use server::launch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();
    launch().await
}