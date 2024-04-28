use actix_web::{get, HttpResponse, Responder};
use redis::AsyncCommands;
use crate::common::cache::conn;


#[get("/cache")]
async fn cache() -> impl Responder{
    let mut cnn = conn().await;
    let _: () = cnn.set("my_key", 42).await.unwrap();
    let result: i32 = cnn.get("my_key").await.unwrap();
    HttpResponse::Ok().body(format!("my_key: {}", result))
}