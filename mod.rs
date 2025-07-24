use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeInfo {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub bid_price: f64,
    pub ask_price: f64,
    pub bid_qty: f64,
    pub ask_qty: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangularArbitrageOpportunity {
    pub id: String,
    pub exchange: String,
    pub path: String,  // "BTC → USDT → ETH → BTC"
    pub pairs: String, // "BTCUSDT, ETHUSDT, ETHBTC"
    pub gross_profit_percentage: f64,
    pub estimated_fees: f64,
    pub net_profit_percentage: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub exchange_id: u32,
    pub min_profit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResponse {
    pub opportunities: Vec<TriangularArbitrageOpportunity>,
    pub total_pairs: usize,
    pub scan_time_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}