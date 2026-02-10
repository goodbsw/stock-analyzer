use tokio::sync::mpsc;
use futures_util::StreamExt;
use async_trait::async_trait;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use crate::model::{BinanceTicker, Exchange, PriceMessage};

pub struct BinanceCollector {
    pub name: String
}

#[async_trait]
impl Exchange for BinanceCollector {
    fn name(&self) -> String {
        self.name.to_string()
    }
    async fn run(&self, tx: mpsc::Sender<PriceMessage>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("wss://stream.binance.us:9443/ws/{}@ticker", &self.name());
        let (ws_stream, _) = connect_async(url).await?;
        println!("Success Binance connection");
        let (_, mut read) = ws_stream.split();

        while let Some(message) = read.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Ok(ticker) = serde_json::from_str::<BinanceTicker>(&text) {
                        if let Ok(price) = ticker.close_price.parse::<f64>() {
                            let _ = tx.send(PriceMessage::Binance(price)).await?;
                        }
                    }
                }
                _ => println!("No data!")
            }
        }
        println!("{} collector stopped!", self.name());
        Ok(())
    }
}