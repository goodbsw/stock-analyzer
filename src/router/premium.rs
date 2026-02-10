use crate::model::AppState;
use axum::{
    extract::State,
    response::IntoResponse
};
use std::sync::Arc;
use serde_json::json;

pub async fn get_premium(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let upbit = *state.upbit_price.read().unwrap();
    let binance = *state.binance_price.read().unwrap();
    let premium = *state.kimchi_premium.read().unwrap();

    axum::Json(json!({
        "upbit": upbit,
        "binance": binance,
        "premium": premium
    }))
}