use std::u32;

use anyhow::Result;
use async_trait::async_trait;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[async_trait]
#[allow(dead_code)]
pub trait CheckFile {
    async fn is_exist(&self, file_key: &str) -> Result<bool>;
}

#[async_trait]
#[allow(dead_code)]
pub trait ModifyFile {
    // Merge all chunks into one file
    async fn merge(&self, file_key: &str, total_chunk: u32) -> Result<()>;
    // Delete all chunks and the directory
    async fn delete(&self, file_key: &str) -> Result<()>;
}

pub struct FileManagerService;

#[async_trait]
impl CheckFile for FileManagerService {
    async fn is_exist(&self, file_key: &str) -> Result<bool> {
        Ok(file_key == "file_key")
    }
}

#[async_trait]
impl ModifyFile for FileManagerService {
    async fn merge(&self, file_key: &str, total_chunk: u32) -> Result<()> {
        let file_dir_path = format!("./storage/{}", file_key);
        let file_path = format!("{}{}", file_dir_path, "original");

        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .await?;

        for i in 0..total_chunk {
            let chunk_path = format!("{}/{}", file_dir_path, i);
            let mut chunk = File::open(chunk_path).await?;

            let mut buffer = Vec::new();
            chunk.read_to_end(&mut buffer).await?;

            file.write_all(&buffer).await?;
        }

        Ok(())
    }

    async fn delete(&self, file_key: &str) -> Result<()> {
        let file_dir_path = format!("./storage/{}", file_key);
        tokio::fs::remove_dir_all(file_dir_path).await?;
        Ok(())
    }
}
