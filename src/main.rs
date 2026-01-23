use serde::Deserialize;
use anyhow::Result;
use yfinance_rs::{Ticker, YfClient};
use yfinance_rs::core::conversions::money_to_f64;

#[derive(Debug, Deserialize)]
struct StockQuote {
    symbol: String,
    price: f64,
    price_target: f64,
    recs_summary: String,
}

async fn fetch_quote(symbol: &str) -> Result<StockQuote> {
    let client = YfClient::default();
    let ticker = Ticker::new(&client, symbol);

    let (info_rs, target_rs, sum_rs) = tokio::join!(
        ticker.quote(),
        ticker.analyst_price_target(None),
        ticker.recommendations_summary()
    );

    let info = info_rs?;
    let target_info = target_rs?;
    let sum_info = sum_rs?;

    let quote = StockQuote{
        symbol: symbol.to_string(),
        price: info.price.as_ref().map(money_to_f64).unwrap_or(0.0),
        price_target: target_info.mean.as_ref().map(money_to_f64).unwrap_or(0.0),
        recs_summary: sum_info
            .mean_rating_text
            .as_deref()
            .unwrap_or("N/A").to_string(),
    };
    Ok(quote)
}

#[tokio::main]
async fn main() -> Result<()> {
    let company_list = vec!["AAPL", "TSLA", "AMZN", "NVDA"];

    let mut handles = vec![];
    for company in company_list {
        let handle = tokio::spawn(async move {
            fetch_quote(&company).await
        });
        handles.push(handle);
    }

    let mut results = vec![];

    for handle in handles {
        match handle.await? {
            Ok(quote) => results.push(quote),
            Err(e) => println!("Error collecting data: {:?}", e),
        }
    }

    results.sort_by(|a, b| {
        let a_upside = (a.price_target / a.price) - 1.0;
        let b_upside = (b.price_target / b.price) - 1.0;
        b_upside.partial_cmp(&a_upside).unwrap()
    });

    println!("🚀Starting stock analysis report!");
    for (i, q) in results.iter().enumerate() {
        let upside = (q.price_target / q.price - 1.0) * 100.0;
        println!(
            "{}. {} (현재: ${:.2} / 목표: ${:.2}) -> 기대수익률: {:.1}%",
            i + 1, q.symbol, q.price, q.price_target, upside
        )
    }

    if let Some(best) = results.first() {
        println!("🥳최고의 추식은 {} 입니다!", best.symbol)
    }

    Ok(())
}
