use crate::exchanges::Exchange;
use crate::models::Ticker;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct KucoinTicker {
    #[serde(rename = "symbol")]
    symbol: String,
    #[serde(rename = "bestBid")]
    bid_price: String,
    #[serde(rename = "bestAsk")]
    ask_price: String,
    #[serde(rename = "bestBidSize")]
    bid_qty: String,
    #[serde(rename = "bestAskSize")]
    ask_qty: String,
}

pub struct KucoinExchange {
    name: String,
    api_url: String,
    id: u32,
    enabled: bool,
}

impl KucoinExchange {
    pub fn new() -> Self {
        Self {
            name: "Kucoin".to_string(),
            api_url: "https://api.kucoin.com".to_string(),
            id: 3,
            enabled: true,
        }
    }
}

#[async_trait]
impl Exchange for KucoinExchange {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u32 {
        self.id
    }

    async fn fetch_tickers(&self) -> Result<Vec<Ticker>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/market/allTickers", self.api_url);
        
        let response = reqwest::get(&url).await?;
        
        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }
        
        let json: serde_json::Value = response.json().await?;
        
        let mut tickers = Vec::new();
        
        if let Some(data) = json.get("data") {
            if let Some(ticker_list) = data.get("ticker") {
                if let Some(ticker_array) = ticker_list.as_array() {
                    for ticker_value in ticker_array {
                        if let Ok(kt) = serde_json::from_value::<KucoinTicker>(ticker_value.clone()) {
                            let (base, quote) = parse_symbol(&kt.symbol);
                            
                            if let (Ok(bid), Ok(ask), Ok(bid_qty), Ok(ask_qty)) = (
                                kt.bid_price.parse::<f64>(),
                                kt.ask_price.parse::<f64>(),
                                kt.bid_qty.parse::<f64>(),
                                kt.ask_qty.parse::<f64>(),
                            ) {
                                let ticker = Ticker {
                                    symbol: kt.symbol.clone(),
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
    if let Some(pos) = symbol.find('-') {
        let base = &symbol[..pos];
        let quote = &symbol[pos + 1..];
        (base.to_string(), quote.to_string())
    } else {
        (symbol.to_string(), "USDT".to_string())
    }
}
