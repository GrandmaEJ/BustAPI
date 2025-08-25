use pyo3::prelude::*;   // PyResult, Python interaction
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::runtime::Runtime;

/// Start the server
pub fn start_server(host: String, port: u16) -> PyResult<()> {
    let rt = Runtime::new().unwrap();

    let addr = format!("{}:{}", host, port).parse().unwrap();

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|_req: Request<Body>| async {
            Ok::<_, Infallible>(Response::new(Body::from("Hello from BustAPI!")))
        }))
    });

    rt.block_on(async {
        let server = Server::bind(&addr).serve(make_svc);
        println!("🚀 BustAPI running on http://{}", addr);
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }
    });

    Ok(())
}
