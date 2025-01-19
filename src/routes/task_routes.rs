use crate::actors::manager::Manager;
use crate::actors::message::{ActivateTask, TrackTaskProgress};
use actix::Addr;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_task(
    manager: web::Data<Addr<Manager>>,
    payload: web::Json<ActivateTask>,
) -> impl Responder {
    let task_message = payload.into_inner();

    let result = manager.send(task_message).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("Task created successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create task"),
    }
}

pub async fn activate_task(
    manager: web::Data<Addr<Manager>>,
    payload: web::Json<ActivateTask>,
) -> impl Responder {
    let task_message = payload.into_inner();

    let result = manager.send(task_message).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("Task activated successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to activate task"),
    }
}

pub async fn track_task_progress(
    manager: web::Data<Addr<Manager>>,
    payload: web::Json<TrackTaskProgress>,
) -> impl Responder {
    let progress_message = payload.into_inner();

    let result = manager.send(progress_message).await;
    match result {
        Ok(progress) => HttpResponse::Ok().json(format!("Task progress: {:?}%", progress)),
        Err(_) => HttpResponse::InternalServerError().json("Failed to track task progress"),
    }
}

pub fn configure_task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .route("/create", web::post().to(create_task))
            .route("/activate", web::post().to(activate_task))
            .route("/progress", web::post().to(track_task_progress)),
    );
}
