pub mod proto {
    tonic::include_proto!("shroom");
}

use proto::{
    shroom_server::{Shroom, ShroomServer},
    HealthCheckRequest, HealthCheckResponse, KeyEvent, StatusRequest, StatusResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct ShroomService;

#[tonic::async_trait]
impl Shroom for ShroomService {
    async fn send_key(
        &self,
        _request: Request<KeyEvent>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn stream_keys(
        &self,
        _request: Request<tonic::Streaming<KeyEvent>>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        Ok(Response::new(HealthCheckResponse { healthy: true }))
    }

    async fn get_status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}

pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("0.0.0.0:{}", port).parse()?;
    let service = ShroomService::default();

    tonic::transport::Server::builder()
        .add_service(ShroomServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
