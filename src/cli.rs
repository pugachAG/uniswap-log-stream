use clap::Parser;

use crate::data::USDC_UNISWAP_ADDRESS;

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// URL of the websocket endpoint,
    /// for example: wss://mainnet.infura.io/ws/v3/YOUR-API-KEY
    #[arg(short, long)]
    pub url: String,

    /// Address of the Uniswap contract to subscribe to,
    #[arg(short, long, default_value = USDC_UNISWAP_ADDRESS)]
    pub address: String,
}

pub fn parse_cli_args() -> CliArgs {
    CliArgs::parse()
}
