mod agent;
use actix_web::{web, App, HttpServer, Responder, HttpRequest};

async fn welcome(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn get_agents() -> impl Responder {
    let mut vec:Vec<agent::common::AgentInfo> = Vec::new();
    println!("get_agents!");
    agent::get_agents(&mut vec);
    return web::Json(vec);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/",web::get().to(welcome))
            .route("/welcome/{name}", web::get().to(welcome))
            .route("/agents", web::get().to(get_agents))

    })
        .bind(("127.0.0.1", 8080)).unwrap()
        .run()
        .await
}