use actix::Actor;
use actix_cors::Cors;
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

use chat::{
    chat_server::ChatServer,
    routes::{ws_connect, ws_connect_with_path},
};

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
    port: u16,
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
        .arg(
            clap::Arg::new("port")
                .long("port")
                .short('p')
                .value_name("PORT")
                .help("Sets the port to listen on")
                .default_value("8081"),
        )
        .get_matches();

    let dbpath = arg_matches.get_one::<String>("dbpath").unwrap();
    let port = arg_matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    let pool = db_init(dbpath)
        .await
        .expect("Failed to initialize database");
    let chat_server = ChatServer::new(pool.clone()).start();

    // TODO: remove this in production
    let jwt_secret = "123456".to_string();
    // let jwt_secret = generate_secret(); // disabled in development

    let app_config = AppConfig {
        port: port.clone(),
        jwt_secret,
    };
    println!(
        "The server is currently listening on localhost:{}.",
        app_config.port
    );
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .app_data(web::Data::new(app_config.clone()))
            .service(index)
            .service(signup)
            .service(login)
            .service(ws_connect_with_path)
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(http_validator))
                    .service(ws_connect)
                    .service(get_history)
                    .service(create_group)
                    .service(list_groups)
                    .service(join_group)
                    .service(leave_group)
                    .default_service(web::route().to(api_default)),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
