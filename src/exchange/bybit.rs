use crate::exchanges::Exchange;
use crate::models::Ticker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BybitTicker {
    #[serde(rename = "symbol")]
    symbol: String,
    #[serde(rename = "bid_price")]
    bid_price: String,
    #[serde(rename = "ask_price")]
    ask_price: String,
    #[serde(rename = "bid_qty")]
    bid_qty: String,
    #[serde(rename = "ask_qty")]
    ask_qty: String,
}

pub struct BybitExchange {
    name: String,
    api_url: String,
    id: u32,
    enabled: bool,
}

impl BybitExchange {
    pub fn new() -> Self {
        Self {
            name: "Bybit".to_string(),
            api_url: "https://api.bybit.com".to_string(),
            id: 2,
            enabled: true,
        }
    }
}

#[async_trait]
impl Exchange for BybitExchange {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.id
    }

    async fn fetch_tickers(&self) -> Result<Vec<Ticker>, Box<dyn std::error::Error>> {
        let url = format!("{}/v5/market/tickers?category=spot", self.api_url);
        
        let response = reqwest::get(&url).await?;
        
        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }
        
        let json: serde_json::Value = response.json().await?;
        
        let mut tickers = Vec::new();
        
        if let Some(result) = json.get("result") {
            if let Some(list) = result.get("list") {
                if let Some(ticker_array) = list.as_array() {
                    for ticker_value in ticker_array {
                        if let Ok(bt) = serde_json::from_value::<BybitTicker>(ticker_value.clone()) {
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
                    }
                }
            }
        }
        
        Ok(tickers)
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

fn parse_symbol(symbol: &str) -> (String, String) {
    let quote_currencies = ["USDT", "USDC", "BTC", "ETH"];
    
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
