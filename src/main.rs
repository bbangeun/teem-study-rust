mod config;
mod errors;
mod panes;

use ::config::Config;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::config::ServerConfig;
use panes::handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ServerConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/panes").route(web::post().to(add_pane)))
    })
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}