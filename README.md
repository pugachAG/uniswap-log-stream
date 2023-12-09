# uniswap-log-stream

## Usage

Use `--help` for details:
```
> cargo run -- --help

Listens for UniswapV3Pool Swap events and stores the decoded data to CSV file

Usage: uniswap-log-stream [OPTIONS] --url <URL> --output <OUTPUT>

Options:
  -u, --url <URL>          URL of the websocket endpoint, for example: wss://mainnet.infura.io/ws/v3/YOUR-API-KEY
  -a, --address <ADDRESS>  Address of the Uniswap contract for subscription [default: 0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640]
  -o, --output <OUTPUT>    File path to persist data in CSV format. The file is truncated if it already exists
  -h, --help               Print help
```

## Testing

Use standard `cargo test`.