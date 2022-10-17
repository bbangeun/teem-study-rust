mod config;
mod errors;
mod panes;

use ::config::Config;
use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use actix_web::http::header;
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

        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/pane/{page_id}").route(web::get().to(get_pane)))
            // .service(web::resource("/pane").route(web::post().to(add_pane)))
            .service(web::resource("/pane").route(web::post().to(save_pane)))
    })
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}