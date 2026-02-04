use tokio::sync::mpsc;
use serde_json::json;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use crate::model::{PriceMessage, UpbitTicker};

pub async fn run(tx: mpsc::Sender<PriceMessage>) {
    let url = "wss://api.upbit.com/websocket/v1";
    let (ws_stream, _) = connect_async(url).await.expect("업비트 연결 실패");
    let (mut write, mut read) = ws_stream.split();

    let subscribe_msg = json!([
        {"ticket":"test"},
        {"type":"ticker","codes":["KRW-BTC"]}
    ]).to_string();

    let _ = write.send(Message::Text(subscribe_msg)).await.expect("UPBIT: Connection Failure");

    while let Some(Ok(Message::Binary(bin))) = read.next().await {
        if let Ok(ticker) = serde_json::from_slice::<UpbitTicker>(&bin) {
            let _ = tx.send(PriceMessage::Upbit(ticker.trade_price)).await;
        }
    }
}