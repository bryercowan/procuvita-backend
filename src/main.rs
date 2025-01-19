use actix_web::{web, App, HttpServer, Responder};

mod actors;
mod routes;

async fn index() -> impl Responder {
    "Hello, Rust backend!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
