pub mod actors;
pub mod routes;
use actix::Actor;
use actix_web::{web, App, HttpServer};
use actors::manager::Manager;
use routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = Manager::new().start();
    let manager_data = web::Data::new(manager);

    HttpServer::new(move || {
        App::new()
            .app_data(manager_data.clone())
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
