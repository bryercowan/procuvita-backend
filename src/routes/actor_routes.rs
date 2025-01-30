use crate::actors::manager::Manager;
use crate::actors::message::{CreateActor, ForwardToActor, GetActorCount};
use actix::Addr;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn create_actor(
    manager: web::Data<Addr<Manager>>,
    payload: web::Json<CreateActor>,
) -> impl Responder {
    let create_msg = payload.into_inner();

    let result = manager
        .send(create_msg)
        .await
        .unwrap_or_else(|_| Err("Failed to create actor".to_string()));

    match result {
        Ok(actor_id) => HttpResponse::Ok().json(json!({
            "message": "Actor created successfully",
            "actor_id": actor_id
        })),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

pub async fn interact_with_actor(
    manager: web::Data<Addr<Manager>>,
    payload: web::Json<ForwardToActor>,
) -> impl Responder {
    let forward_msg = payload.into_inner();

    let result = manager
        .send(forward_msg)
        .await
        .unwrap_or_else(|_| Err("Failed to interact with actor".to_string()));

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

pub async fn list_actors(manager: web::Data<Addr<Manager>>) -> impl Responder {
    match manager.send(GetActorCount).await {
        Ok(count) => HttpResponse::Ok().json(format!("Active actors: {}", count)),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch actor count"),
    }
}

pub fn configure_actor_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/actors")
            .route("/create", web::post().to(create_actor))
            .route("/interact", web::post().to(interact_with_actor))
            .route("/list", web::get().to(list_actors)),
    );
}
