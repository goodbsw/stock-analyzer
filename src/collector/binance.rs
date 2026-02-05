use tokio::sync::mpsc;
use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use crate::model::{PriceMessage, BinanceTicker};

pub async fn run(tx: mpsc::Sender<PriceMessage>) {
    let url = "wss://stream.binance.us:9443/ws/btcusdt@ticker";
    let (ws_stream, _) = connect_async(url).await.expect("Binance Connection Failure");
    println!("Success Binance connection");
    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        // if message is coming
        println!("1. raw message from Binance: {:?}", message);
        match message {
            Ok(Message::Text(text)) => {
                if let Ok(ticker) = serde_json::from_str::<BinanceTicker>(&text) {
                    if let Ok(price) = ticker.close_price.parse::<f64>() {
                        println!("Binance Price: {}", price);
                        let _ = tx.send(PriceMessage::Binance(price)).await;
                    }
                }
            }
            _ => println!("No data!")
        }
    }
}