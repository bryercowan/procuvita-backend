use crate::actors::manager::Manager;
use crate::actors::message::{BroadcastNotification, GetActorCount, QueryActorState};
use actix::Addr;
use actix_web::{web, HttpResponse, Responder};

pub async fn list_all_actors(manager: web::Data<Addr<Manager>>) -> impl Responder {
    let result = manager.send(GetActorCount).await;
    match result {
        Ok(count) => HttpResponse::Ok().json(format!("Total active actors: {}", count)),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch actor count"),
    }
}

pub async fn broadcast_message(
    manager: web::Data<Addr<Manager>>,
    payload: web::Json<BroadcastNotification>,
) -> impl Responder {
    let message = payload.into_inner();

    let result = manager.send(message).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("Broadcast message sent successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to send broadcast message"),
    }
}

pub async fn query_actor_state(
    manager: web::Data<Addr<Manager>>,
    payload: web::Json<QueryActorState>,
) -> impl Responder {
    let query = payload.into_inner();

    let result = manager.send(query).await;
    match result {
        Ok(state) => HttpResponse::Ok().json(state),
        Err(_) => HttpResponse::InternalServerError().json("Failed to query actor state"),
    }
}

pub fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("/actors", web::get().to(list_all_actors))
            .route("/broadcast", web::post().to(broadcast_message))
            .route("/query", web::post().to(query_actor_state)),
    );
}
