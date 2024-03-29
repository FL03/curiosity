/*
    Appellation: curiosity <wasm>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{primitives::*, utils::*};

mod primitives;
mod utils;

use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> AsyncResult {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let server = Server::new(addr);
    server.serve().await?;

    Ok(())
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Server {
    pub address: SocketAddr,
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }
    pub async fn serve(&self) -> AsyncResult {
        let listener = TcpListener::bind(self.address.clone()).await?;
        loop {
            let (stream, _) = listener.accept().await.expect("Listener failed...");

            tokio::task::spawn(async move {
                if let Err(err) = Http::new().serve_connection(stream, service_fn(echo)).await {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
