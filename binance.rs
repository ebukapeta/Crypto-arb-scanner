use crate::exchanges::Exchange;
use crate::models::Ticker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BinanceTicker {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    bid_price: String,
    #[serde(rename = "a")]
    ask_price: String,
    #[serde(rename = "B")]
    bid_qty: String,
    #[serde(rename = "A")]
    ask_qty: String,
}

pub struct BinanceExchange {
    name: String,
    api_url: String,
    id: u32,
    enabled: bool,
}

impl BinanceExchange {
    pub fn new() -> Self {
        Self {
            name: "Binance".to_string(),
            api_url: "https://api.binance.com".to_string(),
            id: 1,
            enabled: true,
        }
    }
}

#[async_trait]
impl Exchange for BinanceExchange {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.id
    }

    async fn fetch_tickers(&self) -> Result<Vec<Ticker>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v3/ticker/bookTicker", self.api_url);
        
        let response = reqwest::get(&url).await?;
        
        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }
        
        let binance_tickers: Vec<BinanceTicker> = response.json().await?;
        
        let mut tickers = Vec::new();
        
        for bt in binance_tickers {
            let (base, quote) = parse_symbol(&bt.symbol);
            
            if let (Ok(bid), Ok(ask), Ok(bid_qty), Ok(ask_qty)) = (
                bt.bid_price.parse::<f64>(),
                bt.ask_price.parse::<f64>(),
                bt.bid_qty.parse::<f64>(),
                bt.ask_qty.parse::<f64>(),
            ) {
                let ticker = Ticker {
                    symbol: bt.symbol.clone(),
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
    let quote_currencies = ["USDT", "USDC", "BUSD", "BTC", "ETH", "BNB"];
    
    for quote in &quote_currencies {
        if symbol.ends_with(quote) {
            let base = symbol.strip_suffix(quote).unwrap_or("");
            return (base.to_string(), quote.to_string());
        }
    }
    
    if symbol.len() > 3 {
        let mid = symbol.len() / 2;
        (symbol[..mid].to_string(), symbol[mid..].to_string())
    } else {
        (symbol.to_string(), "USDT".to_string())
    }
}