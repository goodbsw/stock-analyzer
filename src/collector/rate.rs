use crate::model::AppState;
use tokio::time::{sleep, Duration};

pub async fn run(state: &AppState) {
    loop {
        let current_rate = 1350.00;

        {
            let mut rate_lock = state.rate.write().unwrap();
            *rate_lock = current_rate;
        }
        sleep(Duration::from_secs(3600)).await;
    }
}