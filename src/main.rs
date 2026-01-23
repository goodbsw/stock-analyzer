use serde::Deserialize;
use anyhow::Result;
use yfinance_rs::{Ticker, YfClient};
use yfinance_rs::core::conversions::money_to_f64;

#[derive(Debug, Deserialize)]
struct StockQuote {
    symbol: String,
    price: f64,
}

async fn fetch_quote(symbol: &str) -> Result<StockQuote> {
    let client = YfClient::default();
    let ticker = Ticker::new(&client, symbol);
    let info = ticker.quote().await?;
    let quote = StockQuote{
        symbol: symbol.to_string(),
        price: info.price.as_ref().map(money_to_f64).unwrap_or(0.0)
    };
    Ok(quote)
}

#[tokio::main]
async fn main() -> Result<()> {
    let company_list = vec!["AAPL", "TSLA", "AMZN", "NVDA"];

    let mut handles = vec![];
    for company in company_list {
        let handle = tokio::spawn(async move {
            if let Err(e) = fetch_quote(&company).await {
                eprintln!("Error fetching ticker: {}", e);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let quote = handle.await?;
        println!("Ticker quote: {:#?}", quote);
    }

    println!("Collected all data");
    Ok(())
}
