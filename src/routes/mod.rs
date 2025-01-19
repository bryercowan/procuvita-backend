pub mod actor_routes;
pub mod admin_routes;
pub mod task_routes;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    actor_routes::configure_actor_routes(cfg);
    admin_routes::configure_admin_routes(cfg);
    // task_routes::configure_task_routes(cfg);
}
