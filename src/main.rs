use std::{net::SocketAddr};

use http_body_util::Full;
use hyper::{Request, Response, Uri, body::{self, Bytes}, service::service_fn};
use hyper_util::{rt::{TokioExecutor, TokioIo}, server::conn::auto};
use tokio::{net::TcpListener};

async fn handle(req: Request<body::Incoming>) -> Result<Response<http_body_util::Full<Bytes>>, hyper::Error>{
    let _req =req.into_body();
    let msg=match req.uri().host(){
        Some("/")=> "THIS IS THE OFFICIAL PROXY PROVIDED",
        None => "Nothing is returned",
        Some(_) => "",
    };
    let http_response= Response::new(Full::new(Bytes::from("hi there".to_string())));
    Ok(http_response)
}

#[allow(unused_must_use)]
#[tokio::main]
async fn main() {
    //assinging the ip address
    let ip=SocketAddr::from(([127,0,0,1], 3000));
    //starting the server
    let server=TcpListener::bind(ip).await.expect("unable to bind the ip");
    loop {
        if let Ok((stream, _))=server.accept().await{
            let http2= auto::Builder::new(TokioExecutor::new());
            let io=TokioIo::new(stream);
            http2.serve_connection(io, service_fn(handle)).await;
        }
    }
}
