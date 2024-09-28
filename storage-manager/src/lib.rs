use std::env;
use std::net::{IpAddr, SocketAddr};

use tonic::{transport::Server, Request, Response, Status};

use storage_manager::storage_manage_server::{StorageManage, StorageManageServer};
use storage_manager::{DeleteRequest, MergeRequest, StorageManageReply};

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
        println!("Got a request from {:?}", request.remote_addr());
        let reply = storage_manager::StorageManageReply {
            message: "File merged successfully".to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<StorageManageReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = storage_manager::StorageManageReply {
            message: "File deleted successfully".to_string(),
        };

        Ok(Response::new(reply))
    }
}

//#[tokio::main]
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
