use ethers::types::{Address, H256, I256, U256};

pub const USDC_UNISWAP_ADDRESS: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";

#[derive(Debug)]
pub struct SwapEvent {
    pub transaction_hash: H256,
    pub sender: Address,
    pub recipient: Address,
    pub amount_0: I256,
    pub amount_1: I256,
    pub sqrt_price_x96: U256,
    pub liquidity: u128,
    pub tick: i32,
}
