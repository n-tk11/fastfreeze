use anyhow::Result;
use serde::Serialize;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    ffi::OsString,
};
use structopt::StructOpt;

use super::{
    checkpoint::Checkpoint, extract::Extract, install::Install, run::Run,
    wait::Wait,
    CLI,
};


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
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        println!("Start Listening");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("stream incoming");
            handle_connection(stream);
        }
        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream){
    let cmd_line = "bash,-c,for i in $(seq 100); do echo $i; sleep 1; done";
    let args: Vec<OsString> = cmd_line
        .split(",")
        .map(|arg| OsString::from(arg))
        .collect();
    let demo_run = Run{
        image_url: Some(String::from("file:/tmp/ff-test")),
        app_args: args,
        on_app_ready_cmd: None,
        no_restore: false,
        allow_bad_image_version: false,
        passphrase_file: None,
        preserved_paths: Vec::new(),
        tcp_listen_remap: Vec::new(),
        leave_stopped: false,
        verbose: 3,
        app_name: None,
        no_container: false
    };
    //TODO: seperate path for each command(eg run,checkpoint...)
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    let _ = demo_run.run();
    println!("Finish demo_run");
}
//TODO: Make seperate function to handle each request type