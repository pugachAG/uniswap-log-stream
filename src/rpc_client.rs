use std::sync::Arc;

use anyhow::{Context, Result};
use ethers::abi::Address;
use ethers::contract::stream::EventStreamMeta;
use ethers::contract::{abigen, ContractError, Event};
use ethers::types::Log;
use ethers_providers::{Provider, StreamExt, SubscriptionStream, Ws};

use crate::data::SwapEvent;

type SwapEvents = Event<Arc<Provider<Ws>>, Provider<Ws>, SwapFilter>;
type SwapEventStream<'a> =
    EventStreamMeta<'a, SubscriptionStream<'a, Ws, Log>, SwapFilter, ContractError<Provider<Ws>>>;

abigen!(
    UniswapV3Pool,
    r#"[
        event Swap(address indexed sender, address indexed recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick)
    ]"#
);

/// Simple wrapper on top of ether with a more user-friendly API
pub struct RpcClient {
    events: SwapEvents,
}

impl RpcClient {
    pub async fn connect(ws_url: &str, contract_address: &str) -> Result<Self> {
        let provider = Provider::<Ws>::connect(ws_url).await?;
        let address: Address = contract_address.parse()?;
        let contract = UniswapV3Pool::new(address, Arc::new(provider));
        let events = contract.swap_filter().address(address.into());
        Ok(Self { events })
    }

    pub async fn subscribe(&self) -> Result<EventsSubscription<'_>> {
        let stream = self.events.subscribe().await?.with_meta();
        Ok(EventsSubscription { stream })
    }
}

pub struct EventsSubscription<'a> {
    stream: SwapEventStream<'a>,
}

impl EventsSubscription<'_> {
    pub async fn next_event(&mut self) -> Result<SwapEvent> {
        let (swap_filter, log_meta) = self
            .stream
            .next()
            .await
            .context("unexpected end of stream")??;
        let event = SwapEvent {
            sender: swap_filter.sender,
            recipient: swap_filter.recipient,
            amount_0: swap_filter.amount_0,
            amount_1: swap_filter.amount_1,
            sqrt_price_x96: swap_filter.sqrt_price_x96,
            liquidity: swap_filter.liquidity,
            tick: swap_filter.tick,
            transaction_hash: log_meta.transaction_hash,
        };
        Ok(event)
    }
}
