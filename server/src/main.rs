use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use std::{convert::Infallible, net::SocketAddr};
use tokio::{self, net::TcpListener};

pub mod shutdown;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let shutdown = shutdown::ShutdownListener::new().expect("could not register shutdown listener");

    let addr: SocketAddr = "0.0.0.0:9876".parse().unwrap();
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("could not bind to {}", addr));
    println!("Listening on http://{}", addr);

    loop {
        let client = listener.accept();
        let shut_handle = shutdown.handle();
        tokio::select! {
            res = client => {
                let (stream, c_addr) = res?;
                println!("accepted connection from {:?}", c_addr);

                tokio::spawn(async move {
                    if let Err(http_error) = Http::new().serve_connection(stream, service_fn(hello)).await {
                        println!("got error: {}", http_error);
                    }
                });
            },
            _ = shut_handle => {
                break;
            },
        }
    }

    println!("shutting down...");

    Ok(())
}

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Request {:?}: {:?}", req.method(), req.uri());
    Ok(Response::new(Body::from("Hello World!")))
}
