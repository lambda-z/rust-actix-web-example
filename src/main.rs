mod api;
mod config;
mod common;
mod entity;
mod constant;
mod utils;
mod service;
mod bo;
mod settings;

use crate::settings::{Settings, SETTINGS};
use crate::api::cache_controller::cache;
use std::collections::HashMap;
use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_web::web::Data;
use dotenv::var;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use common::file::print_banner;
use log::{info};
use utils::log::init_logger;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use mongodb::bson::bson;
use printpdf::lopdf::xobject::form;
use crate::common::mongo::Mongo;


/// save file
async fn save_file(text: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("foo.txt").await?;
    file.write_all(text.as_bytes()).await?;
    Ok(())
}


async fn get_data() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://google.com".to_string();
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}


// receive file and save it
async fn upload_file(mut payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let mut file = File::create("test.jpg").await.unwrap();

    while let Some(chunk) = payload.next().await {
        let data = chunk.unwrap();
        file.write_all(&data).await.unwrap_or_else(|e| {
            info!("error: {:?}", e);
        })
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


async fn mongo_client(app_state: Data<AppState>) {
    let uri = std::env::var("MONGO_URI").unwrap_or_else(
        |_| "mongodb://localhost:27017".to_string());
    info!("mongo uri: {:?}", app_state.settings.mongo_url);
    let client = Client::with_uri_str(&app_state.settings.mongo_url).await.unwrap();
    let db = client.database("test");
    let collection = db.collection("test");
    let user = User {
        id: "123".to_string(),
        name: Option::from("Tom".to_string()),
    };
    collection.insert_one(user, None).await.unwrap();
}

struct AppState {
    settings: &'static Settings,
    mongo: &'static Mongo
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = &*SETTINGS;

    lazy_static! {
            static ref URI: String = std::env::var("MONGO_URI").unwrap_or_else(
                |_| "mongodb://localhost:27017".to_string());
            static ref DB_NAME: String  = std::env::var("MONGO_DB").unwrap_or_else(
                |_| "test".to_string());
            static ref  MO: Mongo = Mongo {
                uri: URI.clone(),
                db_name: DB_NAME.clone(),
                client: None,
                db: None
            };
    }

    let app_state = web::Data::new(AppState {
        settings: &*settings,
        mongo: &*MO
    });

    init_logger();
    print_banner().await.unwrap();

    HttpServer::new(move || {
        App::new()
            // config app state
            .app_data(app_state.clone())
            // config cors
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .send_wildcard()
                    .allowed_methods(vec![
                        "GET",
                        "POST",
                        "PATCH",
                        "PUT",
                        "DELETE"]
                    )
                    .max_age(3600)
            )
            // config logger
            .wrap(
                actix_web::middleware::Logger::new(
                    r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#,
                ),
            )
            // config routes
            .configure(config::app::config)
    })
        .bind((&*settings.service_host, settings.service_port))?
        .run()
        .await
}
