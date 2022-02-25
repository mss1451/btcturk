use std::time::Duration;

use btcturk::{http::Client, ApiKeys};
use log::info;
use pretty_assertions::assert_eq;

#[ignore = "requires network and may cause server load"]
#[async_std::test]
async fn general_test() {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut client = Client::new(None, None).unwrap();

    let ticker = client.ticker("BTCUSDT").await.unwrap();
    info!("Received ticker: {:?}", ticker);

    assert_eq!(ticker.pair_normalized, "BTC_USDT");
    assert!(ticker.high >= ticker.low);

    client.set_id(Some("integrity_test"));

    let keys = if let Ok(path) = std::env::var("KEYS_PATH") {
        let key_str = std::fs::read_to_string(path).unwrap();
        let mut lines = key_str.lines();
        let public_key = lines.next().unwrap().to_owned();
        let private_key = lines.next().unwrap().to_owned();
        ApiKeys::new(public_key, private_key).unwrap()
    } else {
        panic!(
            "KEYS_PATH environment var is missing. The key file must consist
            of public key + line break + secret key. Pass the path like this: \
            `KEYS_PATH={{path}} cargo test -- --ignored`"
        );
    };

    client.set_keys(Some(keys));

    let open_orders = client.open_orders("BTCUSDT").await.unwrap();

    for order in open_orders.asks.iter().chain(open_orders.bids.iter()) {
        // Wait a little bit otherwise the server may ban this IP for sending
        // too many requests in a short while.
        // See https://docs.btcturk.com/rate-limits.
        async_std::task::sleep(Duration::from_millis(100)).await;
        client.cancel_order(order.id).await.unwrap();
    }
}
