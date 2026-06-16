pub mod proto {
    tonic::include_proto!("shroom");
}

use proto::{
    HealthCheckRequest, HealthCheckResponse, KeyEvent, StatusRequest, StatusResponse,
    shroom_server::{Shroom, ShroomServer},
};
use std::process::Command;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct ShroomService;

#[tonic::async_trait]
impl Shroom for ShroomService {
    async fn send_key(&self, request: Request<KeyEvent>) -> Result<Response<()>, Status> {
        let event = request.into_inner();
        let state = if event.state == 1 { "ON" } else { "OFF" };
        println!("Key: {} {} (unary)", event.key_code, state);
        Ok(Response::new(()))
    }

    async fn stream_keys(
        &self,
        request: Request<tonic::Streaming<KeyEvent>>,
    ) -> Result<Response<()>, Status> {
        let mut stream = request.into_inner();
        while let Some(event) = stream.next().await {
            let event = event?;
            let state = if event.state == 1 { "ON" } else { "OFF" };
            println!("Key: {} {}", event.key_code, state);
            let key_str = format!("{}:{}", event.key_code, event.state);
            let type_status = Command::new("ydotool")
                .args(["key", &key_str])
                .status()
                .expect("Failed to execute ydotool");

            if !type_status.success() {
                eprintln!("Error: Ensure ydotoold is running and your user has permissions.");
            }
        }
        Ok(Response::new(()))
    }

    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        if let Some(client_addr) = request.remote_addr() {
            println!("Incoming request from client IP: {}", client_addr.ip());
            println!("Client port: {}", client_addr.port());
        } else {
            println!("Could not determine client remote address.");
        }

        Ok(Response::new(HealthCheckResponse { healthy: true }))
    }

    async fn get_status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}

pub async fn run_server(address: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", address, port).parse()?;
    let service = ShroomService::default();

    tonic::transport::Server::builder()
        .add_service(ShroomServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
