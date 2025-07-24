use crate::models::Ticker;
use async_trait::async_trait;

#[async_trait]
pub trait Exchange {
    fn name(&self) -> &str;
    fn id(&self) -> u32;
    async fn fetch_tickers(&self) -> Result<Vec<Ticker>, Box<dyn std::error::Error>>;
    fn is_enabled(&self) -> bool;
}

pub mod binance;
pub mod bybit;
pub mod kucoin;
pub mod gateio;

pub fn get_all_exchanges() -> Vec<Box<dyn Exchange>> {
    vec![
        Box::new(binance::BinanceExchange::new()),
        Box::new(bybit::BybitExchange::new()),
        Box::new(kucoin::KucoinExchange::new()),
        Box::new(gateio::GateIoExchange::new()),
    ]
}

pub fn get_exchange_by_id(id: u32) -> Option<Box<dyn Exchange>> {
    let exchanges = get_all_exchanges();
    exchanges.into_iter().find(|e| e.id() == id)
}
