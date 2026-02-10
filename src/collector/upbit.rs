use tokio::sync::{
    mpsc,
    mpsc::error::SendError};
use serde_json::json;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use async_trait::async_trait;
use crate::model::{PriceMessage, Exchange, UpbitTicker};

pub struct UpbitCollector {
    pub name: String
}

#[async_trait]
impl Exchange for UpbitCollector {
    fn name(&self) -> String {
        self.name.to_string()
    }

    async fn run(&self, tx: mpsc::Sender<PriceMessage>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
        let url = "wss://api.upbit.com/websocket/v1";
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        let subscribe_msg = json!([
            {"ticket":"test"},
            {"type":"ticker","codes":[format!("{}", &self.name())]}
        ]).to_string();

        let _ = write.send(Message::Text(subscribe_msg)).await.expect("UPBIT: Subscribe Failure");

        while let Some(Ok(Message::Binary(bin))) = read.next().await {
            if let Ok(ticker) = serde_json::from_slice::<UpbitTicker>(&bin) {
                let _ = tx.send(PriceMessage::Upbit(ticker.trade_price)).await?;
            }
        }
        Ok(())
    }
}