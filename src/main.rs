use std::net::{SocketAddr};

use hyper::{Request, Response, body, server::conn::http2, service::service_fn};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;

async fn handle(req: Request<body::Incoming>) -> Result<Response<String>, hyper::Error>{
    Ok(Response::new("hello there".to_string()))
}
#[tokio::main]
async fn main() {
    //assinging the ip address
    let ip=SocketAddr::from(([127,0,0,1], 3000));
    //starting the server
    let server=TcpListener::bind(ip).await.expect("unable to bind the ip");

    loop {
        match server.accept().await{
            Ok((stream, _ip))=>{
                let http2=http2::Builder::new(TokioExecutor::new());
                let io=TokioIo::new(stream);
                http2.serve_connection(io, service_fn(handle));
            },
            Err(err) => {
                eprint!("{err:?}");
            },
        }
    }
}
