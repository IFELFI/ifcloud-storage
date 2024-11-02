extern crate file_uploader;
extern crate storage_manager;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tokio::spawn(async move {
        storage_manager::run().await.unwrap();
    });
    file_uploader::run().await;
}
