mod model;
mod helper;
mod collector;
mod router;

use std::sync::{Arc, RwLock};
use tokio::sync::{mpsc, broadcast};
use axum::{
    routing::get,
    Router,
};
use model::{AppState, PriceMessage};
use helper::{update_status, process_and_broadcast};

use crate::{collector::{binance::BinanceCollector, upbit::UpbitCollector}, model::Exchange};

#[tokio::main]
async fn main() {
    let (price_tx, mut price_rx) = mpsc::channel::<PriceMessage>(100);
    let (b_tx, _) = broadcast::channel::<String>(16);

    let shared_state = Arc::new(AppState {
        upbit_price: RwLock::new(0.0),
        binance_price: RwLock::new(0.0),
        kimchi_premium: RwLock::new(0.0),
        rate: RwLock::new(0.0),
        tx: b_tx,
    });

    let collectors: Vec<Box<dyn Exchange>> = vec![
        Box::new(BinanceCollector {name: "btcusd".to_string()}),
        Box::new(UpbitCollector {name: "KRW-BTC".to_string()})
    ];

    // 업비트 수집기 실행 (모듈 호출)
    for c in collectors {
        let tx = price_tx.clone();
        tokio::spawn(async move {
            if let Err(e) = c.run(tx).await {
                eprint!("Collector {} died: {}", c.name(), e);
            }
        });
    }

    // 환율
    let shared_state_for_currency_rate = Arc::clone(&shared_state);
    tokio::spawn(async move {
        collector::rate::run(&shared_state_for_currency_rate).await;
    });

    // Consumer & Server 로직...
    let shared_state_for_ingestor = Arc::clone(&shared_state);
    tokio::spawn(async move {
        while let Some(message) = price_rx.recv().await {
            update_status(message, &shared_state_for_ingestor);
            
            // Calculate Kimchi premium
            if let Err(e) = process_and_broadcast(&shared_state_for_ingestor) {
                eprint!("Error occured while processing prices: {}", e);
            }
        }
    });

    let app = Router::new()
        .route("/price", get(router::premium::get_premium))
        .route("/ws", get(router::websocket::ws_handler))
        .with_state(Arc::clone(&shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Listening on 3001 ..");
    axum::serve(listener, app).await.unwrap();
}