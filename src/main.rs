mod api;
mod config;
mod common;
mod entity;
mod constant;
mod utils;
mod service;
mod bo;

use crate::api::cache_controller::cache;
use std::collections::HashMap;
use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use common::file::print_banner;
use log::{info};
use utils::log::init_logger;

/// save file
async fn save_file(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("foo.txt").await?;
    file.write_all(text.as_bytes()).await?;
    Ok(())
}

async fn get_data() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://www.baidu.com".to_string();
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}


// 接收上传文件
async fn upload_file(mut payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let mut file = File::create("test.jpg").await.unwrap();
    while let Some(chunk) = payload.next().await {
        let data = chunk.unwrap();
        file.write_all(&data).await.unwrap();
    }

    if let Ok(_file) = File::open("test.jpg").await {
        println!("file opened");
        return Ok(HttpResponse::Ok().body("file uploaded.."))
    }

    Ok(HttpResponse::Ok().body("file uploaded"))
}


#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    name: Option<String>,
}


#[derive(Serialize, Deserialize)]
struct SimpleUser {
    id: String,

    #[serde(rename = "name")]
    name: Option<String>,
}



#[get("/login")]
async fn index() -> impl Responder {

    let raw_user = User {
        id: "123".to_string(),
        name: Option::from("Tom".to_string()),
    };

    // 将User转换为SimpleUser
    // let user: SimpleUser = serde_json::from_str(
    //     &serde_json::to_string(&raw_user)
    //     .unwrap()
    // )
    //     .unwrap();

    let user = SimpleUser{
        id: raw_user.id.clone(),
        name: raw_user.name.clone(),
    };

    HttpResponse::Ok().json(user)
}



#[get("/users/{user_id}")]
async fn greet(user_id: web::Path<String>) -> impl Responder {
    let mut user_map: HashMap<String, String> = HashMap::new();
    user_map.insert("145".to_string(), "Tom".to_string());
    user_map.insert("456".to_string(), "Jerry".to_string());
    let username = user_map.get(&user_id.to_string()).unwrap();
    let res = get_data().await.unwrap();
    let ret = save_file(res.clone()).await.unwrap();
    println!("ret: {:?}", ret);
    println!("ret: {:?}", username);
    format!("Hello {res}!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    init_logger();
    dotenv::dotenv().expect("Failed to read .env file");
    print_banner().await.unwrap();
    println!("{}", "v1.0.0");
    // println!("{}", env::var("PORT").unwrap());
    println!("Server running at http://127.0.0.1:9090");
    info!("Server running");
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api/v2")
                    .service(greet)
                    .service(index)
                    .service(cache)
                    .service(web::resource("/upload")
                        .route(
                        web::post().to(upload_file)
                    )
                    )
            )
            .wrap(
                // config cors
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .max_age(3600)
            )
            .wrap(
                actix_web::middleware::Logger::new(
                    r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#,
                ),
            )
            // .wrap(actix_web::middleware::Logger::default())
            .configure(config::app::config)
    })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
