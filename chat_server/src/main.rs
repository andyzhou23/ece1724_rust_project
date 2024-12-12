use actix::Actor;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use clap;
mod db;
use db::db_init;

mod user;
use user::{get_history, login, signup};

mod group;
use group::{create_group, join_group, leave_group, list_groups};

mod jwt;
use jwt::http_validator;
mod chat {
    pub mod chat_server;
    pub mod connection_actor;
    pub mod messages;
    pub mod routes;
}

use chat::{chat_server::ChatServer, routes::ws_connect};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust-powered chat server!")
}

async fn api_default() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "error": "Token is valid, but route not found, check the url."
    }))
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({
        "error": "Route not found, check the url."
    }))
}

#[derive(Clone)]
struct AppConfig {
    jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arg_matches = clap::Command::new("chat_server")
        .arg(
            clap::Arg::new("dbpath")
                .long("dbpath")
                .short('d')
                .value_name("FILE")
                .help("Sets the database file path")
                .default_value("server.db"),
        )
        .get_matches();

    let dbpath = arg_matches.get_one::<String>("dbpath").unwrap();

    let pool = db_init(dbpath)
        .await
        .expect("Failed to initialize database");
    let chat_server = ChatServer::new(pool.clone()).start();
    let jwt_secret = "123456".to_string();
    // let jwt_secret = generate_secret(); // disabled in development
    let app_config = AppConfig { jwt_secret };
    println!("The server is currently listening on localhost:8080.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .service(index)
            .service(signup)
            .service(login)
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(http_validator))
                    .service(get_history)
                    .service(create_group)
                    .service(list_groups)
                    .service(join_group)
                    .service(leave_group)
                    .service(ws_connect)
                    .default_service(web::route().to(api_default)),
            )
            .default_service(web::route().to(not_found))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
