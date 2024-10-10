use std::env;

use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct ManageFileService;

impl ManageFileService {
    pub async fn delete_file(file_key: &String) -> Result<()> {
        // Constants
        let storage_path = env::var("BASE_STORAGE_PATH").unwrap_or("./storage".to_string());

        // Original file path
        let file_dir_path = format!("{}/{}", storage_path, file_key);
        // Chunked file path
        let chunk_dir_path = format!("{}/chunks/{}", file_dir_path, file_key);

        // Remove original file
        tokio::fs::remove_dir_all(file_dir_path).await?;
        // Remove chunked file
        tokio::fs::remove_dir_all(chunk_dir_path).await?;

        Ok(())
    }

    pub async fn merge_chunks(file_key: &String, total_chunk_count: u32) -> Result<()> {
        // Constants
        let original_file_name = "original";
        let storage_path = env::var("BASE_STORAGE_PATH").unwrap_or("./storage".to_string());

        // Chunked file path
        let chunk_dir_path = format!("{}/chunks/{}", storage_path, file_key);
        // Original file path
        let original_file_path = format!("{}/{}/{}", storage_path, file_key, original_file_name);

        // Read or create original file
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(original_file_path)
            .await?;

        // Merge chunks
        for i in 0..total_chunk_count {
            let chunk_path = format!("{}/{}", chunk_dir_path, i);
            let mut chunk = File::open(chunk_path).await?;

            let mut buffer = Vec::new();
            chunk.read_to_end(&mut buffer).await?;

            file.write_all(&buffer).await?;
        }

        // Remove chunked file
        tokio::fs::remove_dir_all(chunk_dir_path).await?;

        Ok(())
    }
}
