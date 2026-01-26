use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use url::Url;

#[tokio::main]
async fn main() {
    // 1. 바이낸스 실시간 스트림 주소 (BTC/USDT 거래 데이터)
    let url = "wss://stream.binance.com:9443/ws/btcusdt@aggTrade";
    let url = Url::parse(url).unwrap();

    println!("바이낸스 서버에 연결 중: {}", url);

    // 2. WebSocket 연결
    let (ws_stream, _) = connect_async(url).await.expect("연결 실패!");
    println!("연결 성공! 실시간 데이터를 수집합니다...");

    let (_, mut read) = ws_stream.split();

    // 3. 쏟아지는 데이터 읽기 (비동기 스트림)
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() {
                    // JSON 데이터를 파싱하기 전, 원본 텍스트를 출력해 봅니다.
                    // 초당 몇 번이나 데이터가 올라오는지 확인해 보세요!
                    println!("수신 데이터: {}", msg.to_text().unwrap());
                }
            }
            Err(e) => eprintln!("에러 발생: {}", e),
        }
    }
}