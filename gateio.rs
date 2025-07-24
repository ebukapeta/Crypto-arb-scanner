use crate::exchanges::Exchange;
use crate::models::Ticker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GateIoTicker {
    #[serde(rename = "currency_pair")]
    symbol: String,
    #[serde(rename = "highest_bid")]
    bid_price: String,
    #[serde(rename = "lowest_ask")]
    ask_price: String,
    #[serde(rename = "bid")]
    bid_qty: String,
    #[serde(rename = "ask")]
    ask_qty: String,
}

pub struct GateIoExchange {
    name: String,
    api_url: String,
    id: u32,
    enabled: bool,
}

impl GateIoExchange {
    pub fn new() -> Self {
        Self {
            name: "Gate.io".to_string(),
            api_url: "https://api.gateio.ws".to_string(),
            id: 4,
            enabled: true,
        }
    }
}

#[async_trait]
impl Exchange for GateIoExchange {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.id
    }

    async fn fetch_tickers(&self) -> Result<Vec<Ticker>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v4/spot/tickers", self.api_url);
        
        let response = reqwest::get(&url).await?;
        
        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }
        
        let gateio_tickers: Vec<GateIoTicker> = response.json().await?;
        
        let mut tickers = Vec::new();
        
        for gt in gateio_tickers {
            let (base, quote) = parse_symbol(&gt.symbol);
            
            if let (Ok(bid), Ok(ask), Ok(bid_qty), Ok(ask_qty)) = (
                gt.bid_price.parse::<f64>(),
                gt.ask_price.parse::<f64>(),
                gt.bid_qty.parse::<f64>(),
                gt.ask_qty.parse::<f64>(),
            ) {
                let ticker = Ticker {
                    symbol: gt.symbol.clone(),
                    base_currency: base,
                    quote_currency: quote,
                    bid_price: bid,
                    ask_price: ask,
                    bid_qty,
                    ask_qty,
                    timestamp: chrono::Utc::now().timestamp_millis() as u64,
                };
                tickers.push(ticker);
            }
        }
        
        Ok(tickers)
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

fn parse_symbol(symbol: &str) -> (String, String) {
    if let Some(pos) = symbol.find('_') {
        let base = &symbol[..pos];
        let quote = &symbol[pos + 1..];
        (base.to_string(), quote.to_string())
    } else {
        (symbol.to_string(), "USDT".to_string())
    }
}