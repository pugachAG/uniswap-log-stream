use std::sync::Arc;

use ethers::abi::Address;
use ethers::contract::stream::EventStreamMeta;
use ethers::contract::{ContractError, Event};
use ethers::types::Log;
use ethers_providers::{Provider, StreamExt, SubscriptionStream, Ws};

use crate::data::{SwapEvent, SwapFilter, UniswapV3Pool};

type SwapEvents = Event<Arc<Provider<Ws>>, Provider<Ws>, SwapFilter>;
type SwapEventStream<'a> =
    EventStreamMeta<'a, SubscriptionStream<'a, Ws, Log>, SwapFilter, ContractError<Provider<Ws>>>;

pub struct RpcClient {
    events: SwapEvents,
}

impl RpcClient {
    pub async fn connect(ws_url: &str, contract_address: &str) -> Self {
        let provider = Provider::<Ws>::connect(ws_url).await.unwrap();
        let address: Address = contract_address.parse().unwrap();
        let contract = UniswapV3Pool::new(address, Arc::new(provider));
        let events = contract.swap_filter().address(address.into());
        Self { events }
    }

    pub async fn subscribe(&self) -> EventsSubscription<'_> {
        let stream = self.events.subscribe().await.unwrap().with_meta();
        EventsSubscription { stream }
    }
}

pub struct EventsSubscription<'a> {
    stream: SwapEventStream<'a>,
}

impl EventsSubscription<'_> {
    pub async fn next_event(&mut self) -> SwapEvent {
        let (data, log_meta) = self.stream.next().await.unwrap().unwrap();
        SwapEvent { data, log_meta }
    }
}
