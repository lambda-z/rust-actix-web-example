

pub(crate) async fn conn() -> redis::aio::Connection {
    let client = redis::Client::open(
        "redis://127.0.0.1:6379/").unwrap();
    let cnn = client.get_tokio_connection().await.unwrap();
    cnn
}