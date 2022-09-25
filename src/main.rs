mod config;
mod handlers;
mod repositories;
mod models;
mod errors;
mod jwt;
mod util;

use crate::config::Config;
use crate::handlers::app_config;
use actix_cors::Cors;
use actix_web::{http::header, middleware, App, HttpServer};
use slog_scope::info;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().unwrap();
    let host = config.server.host;
    let port = config.server.port;
    let server_addr = format!("{}:{}", host, port);
    let server_url = config.server.url;

    info!("Starting server at {}", server_url);

    HttpServer::new(move || {       
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind(server_addr)?
    .run()
    .await
}
