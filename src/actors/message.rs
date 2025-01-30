use actix::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), String>")]
pub struct InteractWithActor {
    pub actor_id: String,
    pub message: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<Uuid, String>")]
pub struct CreateActor {
    pub user_id: String,
    pub name: String,
    pub personality: String,
    pub expertise: String,
    pub goals: Vec<String>,
    pub knowledge_base: String,
    pub picture_url: Option<String>,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<String, String>")]
pub struct InteractWithUser {
    pub user_id: String,
    pub query: String,
}

#[derive(Message, Serialize, Deserialize, Debug)]
#[rtype(result = "Result<(), String>")]
pub struct ActivateTask {
    pub task_id: String,
    pub parameters: Option<serde_json::Value>, // Allows for flexible task input
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), String>")]
pub struct StoreInteraction {
    pub actor_id: String,
    pub interaction_data: serde_json::Value, // Stores embeddings or metadata
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), String>")]
pub struct AssignPersonality {
    pub personality: String,
    pub picture_url: Option<String>, // Optional associated picture
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<u8, String>")] // Progress as a percentage
pub struct TrackTaskProgress {
    pub task_id: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<Vec<serde_json::Value>, String>")]
pub struct FetchHistoricalInteractions {
    pub actor_id: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), String>")]
pub struct BroadcastNotification {
    pub message: String,
    pub recipients: Vec<String>, // Actor or user IDs
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), String>")]
pub struct SaveState {
    pub actor_id: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), String>")]
pub struct LoadState {
    pub actor_id: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<String, String>")] // Debugging output
pub struct QueryActorState {
    pub actor_id: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<String, String>")]
pub struct ForwardToActor {
    pub user_id: String,
    pub actor_id: String,
    pub query: String,
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "usize")]
pub struct GetActorCount;
