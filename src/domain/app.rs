use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::{ReceiverStream, IntervalStream};

use tonic::{transport::Server, Request, Response, Status, Streaming};


use entrybuffer::entry_buffer_server::{EntryBuffer, EntryBufferServer};
use entrybuffer::{RangeRequest, RangeReponse, SchedullerRequest, SchedullerReponse};

use cachebuffer::cache_buffer_client::CacheBufferClient;
use cachebuffer::{KeysRequest, ValueReponse, KeyValueToPut, CreationStatus};
use prost::encoding::key_len;

type ResponseStream = ReceiverStream<Result<RangeReponse, Status>>;


pub mod cachebuffer{
    tonic::include_proto!("cachebuffer");
}
pub mod entrybuffer {
    tonic::include_proto!("entrybuffer");
}

#[derive(Debug)]
pub struct EntryBufferService{
}


#[tonic::async_trait]
impl EntryBuffer for EntryBufferService{

    type ListaHorariosDisponiveisStream = ResponseStream;
    async fn lista_horarios_disponiveis(
        &self,
        req: Request<RangeRequest>,
    ) ->Result<Response<Self::ListaHorariosDisponiveisStream>, Status>
    {
        let channel = tonic::transport::Channel::from_static("http://[::1]:50054")
            .connect()
            .await;

        let mut client = CacheBufferClient::new(channel.unwrap());
        let key: KeysRequest = KeysRequest {
            key: get_key(req.get_ref()),
        };
        let request = tonic::Request::new(key);

        let mut response = client.consulta_cache(request).await?.into_inner();


        let (mut tx, mut rx)  = mpsc::channel(400);
        tokio::spawn(async move {
            while let Some(val) = response.next().await{
                tx.send(try_cast(val.unwrap())).await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn agendar_horario(&self, request: Request<SchedullerRequest>)->Result<Response<SchedullerReponse>, Status>{

        let r: SchedullerRequest=request.get_ref().clone();
        let k: KeyValueToPut = KeyValueToPut {
            init: r.init,
            end: r.end,
            hash: r.hash
        };
        let mut client = CacheBufferClient::connect("http://[::1]:50054").await;
        let request = tonic::Request::new(k);

        let result = client.gravar_cache(request);

        let sched_tatus = result.get_ref();
    }
}

fn get_key(range: &RangeRequest)-> String{
    let mes: &str = range.mes.as_ref().unwarp();
    let ano: &str = range.ano.as_ref().unwarp();
    String::from(mes + ":" +ano)
}

fn try_cast(val: ValueReponse)->Result<RangeReponse, Status>{
    RangeReponse{
        init: val.init,
        end: val.end
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut client = RouteGuideClient::connect("http://[::1]:10000").await;

    let addr = "[::1]:50051".parse()?;


    let entrybuffer = EntryBufferService::default();

    Server::builder()
        .add_service(EntryBufferServer::new(entrybuffer))
        .serve(addr)
        .await?;

    Ok(())
}

