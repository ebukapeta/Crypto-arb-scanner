use dialoguer::{Select, Input, Confirm};
use console::Style;
use crate::exchanges::{Exchange, get_all_exchanges};

pub struct UserInterface;

impl UserInterface {
    pub fn show_welcome() {
        let title_style = Style::new().cyan().bold();
        let subtitle_style = Style::new().yellow();
        
        println!("{}", title_style.apply_to("\n╔════════════════════════════════════════════════════════════════════════════════════════╗"));
        println!("{}", title_style.apply_to("║                     CRYPTO TRIANGULAR ARBITRAGE SCANNER v2.0                           ║"));
        println!("{}", title_style.apply_to("║                           Enhanced with Real-Time Analytics                            ║"));
        println!("{}", title_style.apply_to("╚════════════════════════════════════════════════════════════════════════════════════════╝"));
        println!("{}", subtitle_style.apply_to("\nAdvanced scanner with notifications, logging, and analytics"));
        println!("{}", subtitle_style.apply_to("Select an exchange below to begin scanning...\n"));
    }
    
    pub fn select_exchange() -> Option<u32> {
        let exchanges = get_all_exchanges();
        let enabled_exchanges: Vec<&dyn Exchange> = exchanges
            .iter()
            .filter(|e| e.is_enabled())
            .map(|e| e.as_ref())
            .collect();
        
        if enabled_exchanges.is_empty() {
            println!("No exchanges available!");
            return None;
        }
        
        let exchange_names: Vec<String> = enabled_exchanges
            .iter()
            .map(|e| format!("{}. {}", e.id(), e.name()))
            .collect();
        
        let selection = Select::new()
            .with_prompt("Select Exchange to Scan")
            .items(&exchange_names)
            .default(0)
            .interact();
        
        match selection {
            Ok(index) => Some(enabled_exchanges[index].id()),
            Err(_) => None,
        }
    }
    
    pub fn get_advanced_scan_parameters() -> ScanConfig {
        println!("\n=== ADVANCED SCAN CONFIGURATION ===");
        
        let min_profit: f64 = Input::new()
            .with_prompt("Minimum profit percentage to display")
            .default(0.1)
            .interact()
            .unwrap_or(0.1);
        
        let high_profit_threshold: f64 = Input::new()
            .with_prompt("High profit threshold for notifications (0 for no notifications)")
            .default(1.0)
            .interact()
            .unwrap_or(1.0);
        
        let enable_sound = if high_profit_threshold > 0.0 {
            Confirm::new()
                .with_prompt("Enable sound alerts for high profit opportunities?")
                .default(true)
                .interact()
                .unwrap_or(true)
        } else {
            false
        };
        
        let enable_logging = Confirm::new()
            .with_prompt("Enable detailed logging to file?")
            .default(true)
            .interact()
            .unwrap_or(true);
        
        let enable_csv_export = Confirm::new()
            .with_prompt("Export opportunities to CSV file?")
            .default(true)
            .interact()
            .unwrap_or(true);
        
        let enable_analytics = Confirm::new()
            .with_prompt("Enable real-time analytics?")
            .default(true)
            .interact()
            .unwrap_or(true);
        
        let interval: u64 = Input::new()
            .with_prompt("Scan interval (seconds)")
            .default(10u64)  // Changed to 10 seconds
            .interact()
            .unwrap_or(10);  // Changed to 10 seconds
        
        ScanConfig {
            min_profit,
            high_profit_threshold,
            enable_sound,
            enable_logging,
            enable_csv_export,
            enable_analytics,
            interval,
        }
    }
    
    pub fn show_scanning_message(exchange_name: &str, config: &ScanConfig) {
        let style = Style::new().blue().bold();
        println!("\n{}", style.apply_to(format!("🔍 Scanning {} for arbitrage opportunities...", exchange_name)));
        println!("{}", style.apply_to("Advanced Features:"));
        println!("   • Minimum Profit: {}%", config.min_profit);
        if config.high_profit_threshold > 0.0 {
            println!("   • High Profit Alerts: >{}% {}", config.high_profit_threshold, if config.enable_sound { "🔊" } else { "" });
        }
        if config.enable_logging { println!("   • File Logging: ✅"); }
        if config.enable_csv_export { println!("   • CSV Export: ✅"); }
        if config.enable_analytics { println!("   • Real-time Analytics: ✅"); }
        println!("{}", style.apply_to("\nPress Ctrl+C to stop scanning"));
    }
    
    pub fn show_no_opportunities(min_profit: f64) {
        let style = Style::new().yellow();
        println!("\n{}", style.apply_to(format!("No arbitrage opportunities found above {}%", min_profit)));
    }
    
    pub fn show_error(error: &str) {
        let style = Style::new().red().bold();
        println!("\n{} {}", style.apply_to("❌ Error:"), error);
    }
}

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub min_profit: f64,
    pub high_profit_threshold: f64,
    pub enable_sound: bool,
    pub enable_logging: bool,
    pub enable_csv_export: bool,
    pub enable_analytics: bool,
    pub interval: u64,
}