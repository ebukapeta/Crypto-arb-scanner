use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::Logger};
use actix_files::Files;
use serde::Deserialize;
use std::time::Instant;

mod exchanges;
mod arbitrage;
mod models;
mod ui;

use exchanges::{get_exchange_by_id, Exchange};
use arbitrage::ArbitrageDetector;
use models::{ScanRequest, ScanResponse, ErrorResponse, ExchangeInfo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    println!("🚀 Starting Crypto Arbitrage Web Scanner...");
    println!("🌐 Server running at http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/static", "static/").show_files_listing())
            .route("/", web::get().to(index))
            .route("/api/exchanges", web::get().to(get_exchanges))
            .route("/api/scan", web::post().to(scan_arbitrage))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn index() -> Result<HttpResponse> {
    let html = std::fs::read_to_string("static/index.html")
        .unwrap_or_else(|_| "<h1>Crypto Arbitrage Scanner</h1>".to_string());
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

async fn get_exchanges() -> Result<HttpResponse> {
    let exchanges = exchanges::get_all_exchanges();
    let exchange_info: Vec<ExchangeInfo> = exchanges
        .into_iter()
        .map(|e| ExchangeInfo {
            id: e.id(),
            name: e.name().to_string(),
            enabled: e.is_enabled(),
        })
        .collect();
    
    Ok(HttpResponse::Ok().json(exchange_info))
}

async fn scan_arbitrage(scan_request: web::Json<ScanRequest>) -> Result<HttpResponse> {
    let start_time = Instant::now();
    
    let exchange = match get_exchange_by_id(scan_request.exchange_id) {
        Some(exchange) => exchange,
        None => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                error: "Exchange not found".to_string(),
            }));
        }
    };
    
    match exchange.fetch_tickers().await {
        Ok(tickers) => {
            let opportunities = ArbitrageDetector::find_triangular_opportunities(
                &tickers,
                exchange.name(),
                scan_request.min_profit,
            );
            
            let response = ScanResponse {
                opportunities,
                total_pairs: tickers.len(),
                scan_time_ms: start_time.elapsed().as_millis(),
            };
            
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Failed to fetch tickers: {}", e),
            }))
        }
    }
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
