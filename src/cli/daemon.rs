use anyhow::Result;
use structopt::StructOpt;

use super::{
    checkpoint::Checkpoint,
    extract::Extract,
    install::Install,
    run::{new_from_json, Run},
    wait::Wait,
    CLI,
};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use tokio;

enum Command {
    Run(Run),
    Checkpoint(Checkpoint),
    Extract(Extract),
    Wait(Wait),
    Install(Install),
    Daemon(Daemon),
}

#[derive(StructOpt, PartialEq, Debug, Serialize)]
pub struct Daemon {
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}

impl super::CLI for Daemon {
    fn run(self) -> Result<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(run_http_server()).unwrap();
        Ok(())
    }
}

async fn run_http_server() -> Result<(), hyper::Error> {
    let addr = ([127, 0, 0, 1], 7878).into();
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_connection)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await
}

async fn handle_connection(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/run") => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let instance = new_from_json(body_str);

            let _ = instance.run();
            Ok(Response::builder()
                .status(hyper::StatusCode::OK)
                .body(Body::from("App start successfully"))
                .unwrap())
        },
        (&Method::POST, "/checkpoint") => {
            Ok(Response::builder()
                .status(hyper::StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap())
        },
        _ => {
            Ok(Response::builder()
                .status(hyper::StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap())
        },
    }
}
