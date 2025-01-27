use serde_json::{json, Value};
use std::env;
use supabase_rs::SupabaseClient;

pub struct SupabaseService {
    client: SupabaseClient,
}

impl SupabaseService {
    pub fn new() -> Result<Self, String> {
        let supabase_url = env::var("SUPABASE_URL")
            .map_err(|_| "SUPABASE_URL environment variable not set".to_string())?;
        let supabase_key = env::var("SUPABASE_KEY")
            .map_err(|_| "SUPABASE_KEY environment variable not set".to_string())?;

        let client = SupabaseClient::new(supabase_url, supabase_key).map_err(|e| e.to_string())?;
        Ok(SupabaseService { client })
    }

    pub fn get_client(&self) -> &SupabaseClient {
        &self.client
    }

    pub async fn add_user(&self, email: &str, name: &str) -> Result<String, String> {
        self.client
            .insert(
                "users",
                json!({
                    "email": email,
                    "name": name
                }),
            )
            .await
    }

    pub async fn add_actor(
        &self,
        user_id: &str,
        name: &str,
        personality: &str,
        expertise: &str,
        picture_url: Option<&str>,
    ) -> Result<String, String> {
        self.client
            .insert(
                "actors",
                json!({
                    "user_id": user_id,
                    "name": name,
                    "personality": personality,
                    "expertise": expertise,
                    "picture_url": picture_url,
                }),
            )
            .await
    }

    pub async fn save_chat(
        &self,
        actor_id: &str,
        message: &str,
        embedding: Option<Value>,
    ) -> Result<String, String> {
        self.client
            .insert(
                "chats",
                json!({
                    "actor_id": actor_id,
                    "message": message,
                    "embedding": embedding,
                }),
            )
            .await
    }
}
