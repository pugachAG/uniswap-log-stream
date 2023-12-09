use clap::Parser;

use crate::data::USDC_UNISWAP_ADDRESS;

/// Listens for UniswapV3Pool Swap events and stores
/// the decoded data to CSV file.
#[derive(Parser, Debug)]
pub struct CliArgs {
    /// URL of the websocket endpoint,
    /// for example: wss://mainnet.infura.io/ws/v3/YOUR-API-KEY
    #[arg(short, long)]
    pub url: String,

    /// Address of the Uniswap contract for subscription
    #[arg(short, long, default_value = USDC_UNISWAP_ADDRESS)]
    pub address: String,

    /// File path to persist data in CSV format.
    /// The file is truncated if it already exists.
    #[arg(short, long)]
    pub output: String,
}

pub fn parse_cli_args() -> CliArgs {
    CliArgs::parse()
}
