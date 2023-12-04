mod cli;
mod data;
mod rpc_client;

use cli::parse_cli_args;
use rpc_client::RpcClient;

#[tokio::main]
async fn main() {
    let args = parse_cli_args();
    let client = RpcClient::connect(&args.url, &args.address).await;
    let mut sub = client.subscribe().await;
    loop {
        let event = sub.next_event().await;
        println!("{:?}", event);
    }
}
