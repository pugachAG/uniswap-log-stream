use ethers::contract::{abigen, LogMeta};

pub const USDC_UNISWAP_ADDRESS: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";

abigen!(
    UniswapV3Pool,
    r#"[
        event Swap(address indexed sender, address indexed recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick)
    ]"#
);

pub type SwapEventData = SwapFilter;

#[derive(Debug)]
pub struct SwapEvent {
    pub data: SwapEventData,
    pub log_meta: LogMeta,
}
