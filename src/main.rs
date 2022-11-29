// common
mod config;
mod errors;

// rest
mod panes;

// graphql
mod panel;
mod realtimeperf;

use panel::{PanelQuery};
use realtimeperf::{QueryRoot};
use ::config::Config;
use actix_web::{App, HttpServer, web, route, Responder, get};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web_lab::respond::Html;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::panes::handlers::*;
use crate::config::ServerConfig;
use crate::panel::PanelSchema;
use crate::realtimeperf::PerfSchema;
// use crate::lib::configure_service;

// hello Test
#[get("/hello")]
async fn graphql_hello() -> String {
    "says hello!".to_string()
}

// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    ))
}

// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<PerfSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
    // or req.execute(schema).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ServerConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish();

    let server = HttpServer::new(move || {

        let cors = Cors::default()
            .allowed_origin("*")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(Cors::permissive())
            // .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            // .configure(configure_service) // graphQL
            .service(graphql)
            .service(graphql_playground)
            .service(graphql_hello)
            .service(web::resource("/pane/{page_id}").route(web::get().to(get_pane)))
            // .service(web::resource("/pane").route(web::post().to(add_pane)))
            // .service(web::resource("/pane").route(web::post().to(save_pane)))
    })
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}