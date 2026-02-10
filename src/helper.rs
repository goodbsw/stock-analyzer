use crate::model::{AppState, PriceMessage};
use serde_json::json;

pub fn update_status(message: PriceMessage, state: &AppState) {
    match message {
        PriceMessage::Upbit(price) => {
            let mut upbit_p = state.upbit_price.write().unwrap();
            *upbit_p = price;
        }
        PriceMessage::Binance(price) => {
            let mut binance_p = state.binance_price.write().unwrap();
            *binance_p = price;
        }
    }
}

pub fn process_and_broadcast(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let upbit = *state.upbit_price.read().unwrap();
    let binance = *state.binance_price.read().unwrap();
    let rate = *state.rate.read().unwrap();

    if upbit > 0.0 && binance > 0.0 {
        let premium = ((upbit / (binance * rate)) - 1.0) * 100.0;

        {
            let mut kimch_premium = state.kimchi_premium.write().unwrap();
            *kimch_premium = premium;
        }

        let payload = json!({
            "binance": binance,
            "upbit": upbit,
            "kimchi_premium": premium,
            "rate": rate
        }).to_string();

        let _ = state.tx.send(payload);
    }
    Ok(())
}