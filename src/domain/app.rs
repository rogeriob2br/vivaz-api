use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use entrybuffer::entrybuffer_server::{EntryBuffer, EntryBufferServer};
use entrybuffer::{RangeRequest, Range, RangeReponse, SchedullerRequest, SchedullerReponse};
use cachebuffer::cachebuffer_client::CacheBufferClient;
use cachebuffer::{KeysRequest, Range, ValueReponse, KeyValueToPut, Status};

pub mod entrybuffer {
    tonic::include_proto!("entrybuffer");
}

#[derive#(debug)]
pub struct EntryBufferService{

}


#[tonic::async_trait]
impl EntryBuffer for EntryBufferService{
    async fn lista_horarios_disponiveis(
        &self,
        request: Request<HelloRequest>,
    ){
        let mut client = GreeterClient::connect("http://[::1]:50054").await?;
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });
        let response = client.say_hello(request).await?;
    }
    async fn agendar_horario(){

    }
}

pub mod cachebuffer{
    tonic::include_proto!("entrybuffer");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{



    let addr = "[::1]:50051".parse()?;


    let entrybuffer = EntryBufferService::default();

    Server::builder()
        .add_service(EntryBufferServer::new(entrybuffer))
        .serve(addr)
        .await?;

    Ok(())
}

