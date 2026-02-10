use std::sync::RwLock;
use tokio::sync::{mpsc, broadcast};
use serde::Deserialize;
use async_trait::async_trait;

pub struct AppState {
    pub upbit_price: RwLock<f64>,
    pub binance_price: RwLock<f64>,
    pub kimchi_premium: RwLock<f64>,
    pub rate: RwLock<f64>,
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

#[async_trait]
pub trait Exchange: Send + Sync {
    fn name(&self) -> String;
    async fn run(&self, tx: mpsc::Sender<PriceMessage>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}