use crate::actors::message::*;
use actix::prelude::*;
use reqwest::Client;
use serde_json::Value;
use std::env;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserActor {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub personality: String, // Actor's tone (e.g., motivational, empathetic)
    pub picture_url: Option<String>,
    pub expertise: String,      // Area of expertise (e.g., "Fitness", "Career")
    pub goals: Vec<String>,     // Array of goals the actor is helping the user achieve
    pub knowledge_base: String, // Domain-specific tips or knowledge
}

impl UserActor {
    pub fn new(
        id: Uuid,
        user_id: String,
        name: String,
        personality: String,
        picture_url: Option<String>,
        expertise: String,
        goals: Vec<String>,
        knowledge_base: String,
    ) -> Self {
        UserActor {
            id,
            user_id,
            name,
            personality,
            picture_url,
            expertise,
            goals,
            knowledge_base,
        }
    }

    pub async fn fetch_response_from_openai(&self, user_query: String) -> Result<String, String> {
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| "OpenAI API key not found")?;
        let client = Client::new();
        let endpoint = "https://api.openai.com/v1/completions";

        // Construct the AI assistant prompt
        let goal_list = self.goals.join(", ");
        let full_prompt = format!(
            "You are a {expertise} life coach with a {personality} personality. \
            You are helping the user achieve the following goals: {goals}. \
            Use your knowledge base: {knowledge_base}. \
            Respond to this user query: {user_query}",
            expertise = self.expertise,
            personality = self.personality,
            goals = goal_list,
            knowledge_base = self.knowledge_base,
            user_query = user_query
        );

        let body = serde_json::json!({
            "model": "text-davinci-003",
            "prompt": full_prompt,
            "max_tokens": 150,
            "temperature": 0.7
        });

        let response = client
            .post(endpoint)
            .bearer_auth(api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let response_json: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        response_json["choices"][0]["text"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| "No response text found".into())
    }

    async fn store_chat_in_vector_db(
        &self,
        user_id: &str,
        message: &str,
        response: &str,
    ) -> Result<(), String> {
        let api_key = env::var("PINECONE_API_KEY").map_err(|_| "Pinecone API key not found")?;
        let client = Client::new();
        let endpoint = "https://your-pinecone-index-url/vectors/upsert";

        // Create embedding data
        let embedding_data = serde_json::json!({
            "vectors": [
                {
                    "id": format!("chat-{}-{}", user_id, Uuid::new_v4()),
                    "values": self.generate_embedding(message).await?,
                    "metadata": {
                        "user_id": user_id,
                        "actor_id": self.id,
                        "query": message,
                        "response": response,
                    }
                }
            ]
        });

        let response = client
            .post(endpoint)
            .header("Api-Key", api_key)
            .json(&embedding_data)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!(
                "Failed to store interaction: {}",
                response.text().await.unwrap_or_default()
            ))
        }
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        // Placeholder for actual embedding generation (e.g., OpenAI, local embedding API)
        Ok(vec![0.1, 0.2, 0.3]) // Example embedding values
    }
}

impl Actor for UserActor {
    type Context = Context<Self>;
}

impl Handler<InteractWithUser> for UserActor {
    type Result = ResponseFuture<Result<String, String>>;

    fn handle(&mut self, msg: InteractWithUser, _: &mut Context<Self>) -> Self::Result {
        println!("Handle InteractWithUser");
        let user_query = msg.query.clone();
        let user_id = self.user_id.clone();
        let actor_id = self.id.clone();
        let personality = self.personality.clone();
        let expertise = self.expertise.clone();
        let goals = self.goals.clone();
        let knowledge_base = self.knowledge_base.clone();

        println!("what is poppin");
        Box::pin(async move {
            let full_prompt = format!(
                "You are a {expertise} life coach with a {personality} personality. \
                You are helping the user achieve the following goals: {goals}. \
                Use your knowledge base: {knowledge_base}. \
                Respond to this user query: {user_query}",
                expertise = expertise,
                personality = personality,
                goals = goals.join(", "),
                knowledge_base = knowledge_base,
                user_query = user_query
            );

            let api_key = env::var("OPENAI_API_KEY").map_err(|_| "OpenAI API key not found")?;
            let client = Client::new();
            let endpoint = "https://api.openai.com/v1/chat/completions";

            let body = serde_json::json!({
                "model": "gpt-4o-2024-11-20",
                "messages": [{
                    "role": "system",
                    "content": format!(
                        "You are a {} life coach with a {} personality. You are helping the user achieve the following goals: {}. Use your knowledge base: {}.",
                        expertise, personality, goals.join(", "), knowledge_base
                    )
                },
                {
                    "role": "user",
                    "content": user_query
                }],
                "max_tokens": 150,
                "temperature": 0.7
            });

            let response = client
                .post(endpoint)
                .bearer_auth(api_key)
                .json(&body)
                .send()
                .await
                .map_err(|e| {
                    println!("OpenAI request error: {:?}", e);
                    format!("Request failed: {}", e)
                })?;
            println!("OpenAI Response Status: {:?}", response.status());
            println!("OpenAI Response Headers: {:?}", response.headers());
            println!("1");

            let response_json: Value = response.json().await.map_err(|e| {
                println!("OpenAI request error: {:?}", e);
                format!("Request failed: {}", e)
            })?;
            println!("2");
            println!("Full JSON response: {:?}", response_json);

            let response_text: String = response_json["choices"][0]["message"]["content"]
                .as_str()
                .ok_or_else(|| String::from("No response text found"))?
                .to_string();
            println!("3");

            // Store chat in vector DB
            let embedding_data = serde_json::json!({
                "vectors": [
                    {
                        "id": format!("chat-{}-{}", user_id, uuid::Uuid::new_v4()),
                        "values": vec![0.1, 0.2, 0.3], // Replace with real embeddings
                        "metadata": {
                            "user_id": user_id,
                            "actor_id": actor_id,
                            "query": user_query,
                            "response": response_text,
                        }
                    }
                ]
            });

            match env::var("PINECONE_API_KEY") {
                Ok(pinecone_key) => {
                    let vector_response = client
                        .post("https://your-pinecone-index-url/vectors/upsert")
                        .header("Api-Key", pinecone_key)
                        .json(&embedding_data)
                        .send()
                        .await;

                    if let Err(e) = vector_response {
                        println!("Warning: Failed to store chat in vector database: {}", e);
                    }
                }
                Err(e) => {
                    println!("Warning: Pinecone API key not found: {}", e);
                }
            }

            println!("{}", response_text);

            Ok(response_text)
        })
    }
}
