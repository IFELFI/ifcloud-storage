use std::env;
use std::net::{IpAddr, SocketAddr};

use services::ManageFileService;
use tonic::{transport::Server, Request, Response, Status};

use storage_manager::storage_manage_server::{StorageManage, StorageManageServer};
use storage_manager::{DeleteRequest, MergeRequest, StorageManageReply};

mod services;

pub mod storage_manager {
    tonic::include_proto!("storage_manager");
}

#[derive(Default)]
pub struct StorageManageService {}

#[tonic::async_trait]
impl StorageManage for StorageManageService {
    async fn merge(
        &self,
        request: Request<MergeRequest>,
    ) -> Result<Response<StorageManageReply>, Status> {
        let request = request.into_inner();
        let file_key = request.file_key;
        let total_chunk_count = request.total_chunk_count;

        let mut reply = storage_manager::StorageManageReply {
            success: false,
            message: "File merging failed".to_string(),
        };

        let result = ManageFileService::merge_chunks(&file_key, total_chunk_count).await;

        match result {
            Ok(_) => {
                reply.success = true;
                reply.message = "File merged successfully".to_string();
            },
            Err(e) => reply.message = format!("File merging failed: {}", e),
        }

        Ok(Response::new(reply))
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<StorageManageReply>, Status> {
        let file_key = &request.get_ref().file_key;

        let mut reply = storage_manager::StorageManageReply {
            success: false,
            message: "File deletion failed".to_string(),
        };

        let result = ManageFileService::delete_file(file_key).await;

        match result {
            Ok(_) => {
                reply.success = true;
                reply.message = "File deleted successfully".to_string();
            },
            Err(e) => reply.message = format!("File deletion failed: {}", e),
        }

        Ok(Response::new(reply))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from((
        env::var("RPC_HOST")
            .unwrap_or("127.0.0.1".to_string())
            .parse::<IpAddr>()
            .unwrap(),
        env::var("RPC_PORT")
            .unwrap_or("3001".to_string())
            .parse::<u16>()
            .unwrap(),
    ));
    let manager = StorageManageService::default();

    println!("RPC server is running on http://{}", addr);
    Server::builder()
        .add_service(StorageManageServer::new(manager))
        .serve(addr)
        .await?;

    Ok(())
}
