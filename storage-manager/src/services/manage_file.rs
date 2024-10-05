use std::env;

use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct ManageFileService;

impl ManageFileService {
    pub async fn delete_file(file_key: &String) -> Result<()> {
        let storage_path = env::var("BASE_STORAGE_PATH").unwrap_or("./storage".to_string());
        let file_dir_path = format!("{}/{}", storage_path, file_key);
        tokio::fs::remove_dir_all(file_dir_path).await?;

        Ok(())
    }

    pub async fn merge_chunks(file_key: &String, total_chunk_count: u32) -> Result<()> {
        let storage_path = env::var("BASE_STORAGE_PATH").unwrap_or("./storage".to_string());
        let file_dir_path = format!("{}/{}", storage_path, file_key);
        let file_path = format!("{}{}", file_dir_path, "original");

        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .await?;

        for i in 0..total_chunk_count {
            let chunk_path = format!("{}/{}", file_dir_path, i);
            let mut chunk = File::open(chunk_path).await?;

            let mut buffer = Vec::new();
            chunk.read_to_end(&mut buffer).await?;

            file.write_all(&buffer).await?;
        }

        Ok(())
    }
}
