use actix::Actor;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap;
mod db;
use db::db_init;
use actix_cors::Cors;

mod user;
use user::{get_history, login, signup};

mod group;
use group::{create_group, join_group, leave_group, list_groups};

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

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(serde_json::json!({
        "error": "Route not found, check the url."
    }))
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

    println!("The server is currently listening on localhost:8081.");
    HttpServer::new(move || {
        App::new()
        .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(chat_server.clone()))
            .service(index)
            .service(signup)
            .service(login)
            .service(get_history)
            .service(create_group)
            .service(list_groups)
            .service(join_group)
            .service(leave_group)
            .service(ws_connect)
            .default_service(web::route().to(not_found))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
