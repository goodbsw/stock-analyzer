mod model;
mod collector;

use std::sync::{Arc, RwLock};
use tokio::sync::{mpsc, broadcast};
use model::{AppState, PriceMessage};

#[tokio::main]
async fn main() {
    let (price_tx, mut price_rx) = mpsc::channel::<PriceMessage>(100);
    let (b_tx, _) = broadcast::channel::<String>(16);

    let shared_state = Arc::new(AppState {
        upbit_price: RwLock::new(0.0),
        binance_price: RwLock::new(0.0),
        kimchi_premium: RwLock::new(0.0),
        tx: b_tx,
    });

    // 업비트 수집기 실행 (모듈 호출)
    let tx_upbit = price_tx.clone();
    tokio::spawn(async move {
        collector::upbit::run(tx_upbit).await;
    });

    // 바이낸스 수집기도 이런 식으로 추가될 겁니다.
    let tx_binance = price_tx.clone();
    tokio::spawn(async move {
        collector::binance::run(tx_binance).await;
    });

    // Consumer & Server 로직...
    let shared_state_for_ingestor = Arc::clone(&shared_state);
    tokio::spawn(async move {
        while let Some(message) = price_rx.recv().await {
            match message {
                PriceMessage::Upbit(price) => {
                    let mut upbit_p = shared_state_for_ingestor.upbit_price.write().unwrap();
                    *upbit_p = price;
                    println!("The latest price of Upbit is {}", *upbit_p)
                }
                PriceMessage::Binance(price) => {
                    let mut binance_p = shared_state_for_ingestor.binance_price.write().unwrap();
                    *binance_p = price;
                    println!("The latest price of Binance is {}", *binance_p)
                }
            }
        }
    });

    // 영원히 끝나지 않는 대기
    tokio::signal::ctrl_c().await.unwrap();
}