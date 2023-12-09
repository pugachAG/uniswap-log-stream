mod cli;
mod data;
mod persister;
mod rpc_client;

use std::io::{stderr, Write};
use std::process::exit;

use anyhow::Context;
use cli::parse_cli_args;
use rpc_client::RpcClient;

use crate::persister::Persister;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{:?}", err);
        exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let args = parse_cli_args();
    eprintln!("Openning file {}", args.output);
    let mut persister = Persister::create(&args.output).with_context(|| "Failed to open file")?;
    eprintln!("Subscribing to {} on {}", args.address, args.url);
    let rpc_client = RpcClient::connect(&args.url, &args.address)
        .await
        .with_context(|| "Failed to connect to json_rpc instance")?;
    let mut subscription = rpc_client
        .subscribe()
        .await
        .with_context(|| "Failed to subscribe to events stream")?;
    loop {
        eprint!("Waiting for the next event...");
        stderr().flush()?;
        let event = subscription
            .next_event()
            .await
            .with_context(|| "Failed to receive next event")?;
        eprintln!("\rReceived event: {:?}", event);
        persister
            .write(&event)
            .with_context(|| "Failed to persist event")?;
    }
}
