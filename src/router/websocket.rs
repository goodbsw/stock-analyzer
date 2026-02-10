use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse
};
use std::sync::Arc;
use crate::model::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: axum::extract::ws::WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe(); // 브로드캐스트 채널 구독 시작

    while let Ok(msg) = rx.recv().await {
        println!("data from subscript: {:#?}", msg);
        // 채널에 새로운 김프 데이터가 들어오면 웹소켓으로 전송
        if socket.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
            break; // 클라이언트가 접속을 끊으면 루프 탈출
        }
    }
}