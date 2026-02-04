use std::sync::RwLock;
use tokio::sync::broadcast;
use serde::Deserialize;

pub struct AppState {
    pub upbit_price: RwLock<f64>,
    pub binance_price: RwLock<f64>,
    pub kimchi_premium: RwLock<f64>,
    pub tx: broadcast::Sender<String>,
}

pub enum PriceMessage {
    Upbit(f64),
    Binance(f64),
}

#[derive(Debug, Deserialize)]
pub struct UpbitTicker {
    pub trade_price: f64,
}

#[derive(Debug, Deserialize)]
pub struct BinanceTicker {
    #[serde(rename = "c")]
    pub close_price: String,
}