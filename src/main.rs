use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde_json::json;
use serde::Deserialize;
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
struct UpbitTicker {
    code: String,
    trade_price: f64,      // 현재가
    high_price: f64,       // 고가
    low_price: f64,        // 저가
    acc_trade_volume_24h: f64, // 24시간 거래량
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(1000);
    // 업비트 웹소켓 주소
    let url = "wss://api.upbit.com/websocket/v1";
    let (ws_stream, _) = connect_async(url).await.expect("연결 실패");
    let (mut write, mut read) = ws_stream.split();

    // 업비트 구독 형식 (바이낸스와 약간 다릅니다)
    let subscribe_msg = json!([
        {"ticket":"test"},
        {"type":"ticker","codes":["KRW-BTC"]} // 비트코인 시세 구독
    ]).to_string();

    write.send(Message::Text(subscribe_msg)).await.expect("구독 실패");

    tokio::spawn(async move {
       while let Some(bin) = rx.recv().await {
           if let Ok(ticker) = serde_json::from_slice::<UpbitTicker>(&bin) {
               println!("✅ Trade Price: {}, high price {} | row price {}", ticker.trade_price, ticker.high_price, ticker.low_price);
           }
       }
    });

    while let Some(message) = read.next().await {
        if let Ok(Message::Binary(bin)) = message {
            let _ = tx.send(bin).await;
        }
    }
}