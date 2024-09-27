use tonic::{transport::Server, Request, Response, Status};

use storage_manager::storage_manage_server::{StorageManage, StorageManageServer};
use storage_manager::{DeleteRequest, MergeRequest, StorageManageResponse};

pub mod storage_manager {
    tonic::include_proto!("storage_manager");
}

#[derive(Debug, Default)]
pub struct StorageManageService {}

#[tonic::async_trait]
impl StorageManage for StorageManageService {
    async fn merge(
        &self,
        request: Request<MergeRequest>,
    ) -> Result<Response<StorageManageResponse>, Status> {
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<StorageManageResponse>, Status> {
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
