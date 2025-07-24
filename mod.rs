use crate::models::{Ticker, TriangularArbitrageOpportunity};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

pub struct ArbitrageDetector;

impl ArbitrageDetector {
    pub fn find_triangular_opportunities(
        tickers: &[Ticker],
        exchange_name: &str,
        min_profit: f64,
    ) -> Vec<TriangularArbitrageOpportunity> {
        let mut opportunities = Vec::new();
        
        let mut symbol_map: HashMap<String, &Ticker> = HashMap::new();
        for ticker in tickers {
            symbol_map.insert(ticker.symbol.clone(), ticker);
        }
        
        let mut quote_groups: HashMap<String, Vec<&Ticker>> = HashMap::new();
        for ticker in tickers {
            quote_groups.entry(ticker.quote_currency.clone())
                        .or_insert_with(Vec::new)
                        .push(ticker);
        }
        
        for base_ticker in tickers {
            let base_currency = &base_ticker.base_currency;
            let quote_currency = &base_ticker.quote_currency;
            
            if let Some(second_legs) = quote_groups.get(quote_currency) {
                for second_ticker in second_legs {
                    let intermediate_currency = &second_ticker.base_currency;
                    
                    let reverse_symbol = format!("{}{}", intermediate_currency, base_currency);
                    let forward_symbol = format!("{}{}", base_currency, intermediate_currency);
                    
                    if let Some(third_ticker) = symbol_map.get(&reverse_symbol) {
                        if let Some(opportunity) = Self::calculate_profit(
                            base_ticker, second_ticker, third_ticker,
                            exchange_name, min_profit
                        ) {
                            opportunities.push(opportunity);
                        }
                    } else if let Some(third_ticker) = symbol_map.get(&forward_symbol) {
                        if let Some(opportunity) = Self::calculate_profit_reverse(
                            base_ticker, second_ticker, third_ticker,
                            exchange_name, min_profit
                        ) {
                            opportunities.push(opportunity);
                        }
                    }
                }
            }
        }
        
        opportunities.sort_by(|a, b| b.net_profit_percentage.partial_cmp(&a.net_profit_percentage).unwrap());
        opportunities.into_iter().take(20).collect()
    }
    
    fn calculate_profit(
        first: &Ticker,
        second: &Ticker,
        third: &Ticker,
        exchange: &str,
        min_profit: f64,
    ) -> Option<TriangularArbitrageOpportunity> {
        let buy_first = first.ask_price;
        let buy_second = second.ask_price;
        let sell_third = third.bid_price;
        
        let initial_amount = 1.0;
        let amount_after_first = initial_amount / buy_first;
        let amount_after_second = amount_after_first / buy_second;
        let final_amount = amount_after_second * sell_third;
        
        let gross_profit_percentage = ((final_amount - initial_amount) / initial_amount) * 100.0;
        let estimated_fees = 0.1 * 3.0; // 0.1% per trade * 3 trades = 0.3%
        let net_profit_percentage = gross_profit_percentage - estimated_fees;
        
        if net_profit_percentage > min_profit {
            Some(TriangularArbitrageOpportunity {
                id: Uuid::new_v4().to_string(),
                exchange: exchange.to_string(),
                path: format!("{} → {} → {} → {}", 
                             first.base_currency,
                             first.quote_currency, 
                             second.base_currency,
                             first.base_currency),
                pairs: format!("{}, {}, {}", 
                              first.symbol,
                              second.symbol, 
                              third.symbol),
                gross_profit_percentage,
                estimated_fees,
                net_profit_percentage,
                timestamp: Utc::now(),
            })
        } else {
            None
        }
    }
    
    fn calculate_profit_reverse(
        first: &Ticker,
        second: &Ticker,
        third: &Ticker,
        exchange: &str,
        min_profit: f64,
    ) -> Option<TriangularArbitrageOpportunity> {
        let buy_first = first.ask_price;
        let buy_second = second.ask_price;
        let buy_third = third.ask_price;
        
        let initial_amount = 1.0;
        let amount_after_first = initial_amount / buy_first;
        let amount_after_second = amount_after_first / buy_second;
        let final_amount = amount_after_second / buy_third;
        
        let gross_profit_percentage = ((final_amount - initial_amount) / initial_amount) * 100.0;
        let estimated_fees = 0.1 * 3.0; // 0.1% per trade * 3 trades = 0.3%
        let net_profit_percentage = gross_profit_percentage - estimated_fees;
        
        if net_profit_percentage > min_profit {
            Some(TriangularArbitrageOpportunity {
                id: Uuid::new_v4().to_string(),
                exchange: exchange.to_string(),
                path: format!("{} → {} → {} → {}", 
                             first.base_currency,
                             first.quote_currency, 
                             second.base_currency,
                             first.base_currency),
                pairs: format!("{}, {}, {}", 
                              first.symbol,
                              second.symbol, 
                              third.symbol),
                gross_profit_percentage,
                estimated_fees,
                net_profit_percentage,
                timestamp: Utc::now(),
            })
        } else {
            None
        }
    }
}