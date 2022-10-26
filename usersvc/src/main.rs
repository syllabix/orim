use actix_cors::Cors;

use std::io::{Error, ErrorKind};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

mod handler;
mod user;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "debug");

    // let log_level = std::env::var("RUST_LOG").unwrap_or("debug".to_string());
    // std::env::set_var("RUST_LOG", log_level);

    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .map(|ps| ps.parse::<u16>().expect("Invalid PORT specified"))
        .unwrap_or(8080);

    env_logger::init();

    log::info!("starting user service server at {}:{}...", &host, &port);
    let user_registry = web::Data::new(user::Registry::new());

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(cors_config())
            .wrap(logger)
            .route("/healthz", web::get().to(handler::health_check))
            .service(
                web::scope("/v1/user")
                    .app_data(user_registry.clone())
                    .route("", web::put().to(handler::user::create))
                    .route("", web::get().to(handler::user::get_all))
                    .route("/{id}", web::get().to(handler::user::get)),
            )
    })
    .bind((host, port))?
    .run()
    .await
    .map_err(|e| Error::new(ErrorKind::Other, e))?;

    Ok(())
}

fn cors_config() -> Cors {
    let cors_allow_origin =
        std::env::var("CORS_ALLOW_ORIGIN").unwrap_or("http://localhost:3000".to_string());

    Cors::default()
        .allowed_origin(cors_allow_origin.as_str())
        .allowed_methods(vec!["GET", "PUT"])
        .allow_any_header()
        .supports_credentials()
        .max_age(3600)
}