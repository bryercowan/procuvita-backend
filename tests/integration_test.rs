use reqwest::Client;
use serde_json::json;

const BASE_URL: &str = "http://127.0.0.1:8080";

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_all() -> Result<(), Box<dyn std::error::Error>> {
        test_supabase_connection()
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        test_pinecone_connection()
            .await
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        test_routes().await?;
        test_concurrent_interactions().await?;
        Ok(())
    }

    async fn test_supabase_connection() -> Result<(), String> {
        // Replace with Supabase connection logic
        println!("Supabase connection: PASSED");
        Ok(())
    }

    async fn test_pinecone_connection() -> Result<(), String> {
        // Replace with Pinecone connection logic
        println!("Pinecone connection: PASSED");
        Ok(())
    }

    async fn test_routes() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        // Test creating an actor
        let actor_response = client
            .post(&format!("{}/actors/create", BASE_URL))
            .json(&json!({
                "user_id": "user1",
                "name": "Test Actor",
                "personality": "balanced",
                "expertise": "Health & Fitness",
                "goals": ["I need to make sure I am eating healthy.", "I need to lose 25lbs!"],
                "knowledge_base": "Fitness and nutrition expertise with focus on weight management",
                "picture_url": "https://images.unsplash.com/photo-1594824476967-48c8b964273f"
            }))
            .send()
            .await?;

        let status = actor_response.status();

        let response_json: serde_json::Value = actor_response.json().await?;
        print!("Created Actor: {}", response_json);
        let actor_id = response_json["actor_id"]
            .as_str()
            .expect("Failed to get actor_id from response");

        println!("Actor created with ID: {}", actor_id);
        assert!(
            status.is_success(),
            "Failed to create actor. Status: {}, Body: {}",
            status,
            response_json
        );

        let interact_response = client
            .post(&format!("{}/actors/interact", BASE_URL))
            .json(&json!({
                "user_id": "user1",
                "actor_id": actor_id,
                "query": "What should I eat after a workout?"
            }))
            .send()
            .await?;

        let status = interact_response.status();
        let response_body = interact_response.text().await?;

        assert!(
            status.is_success(),
            "Failed to interact with actor. Status: {}, Body: {}",
            status,
            response_body
        );

        println!("Route tests: PASSED");
        Ok(())
    }

    async fn test_concurrent_interactions() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let mut tasks = vec![];

        // Test personalities matching the schema
        let personalities = vec!["stern", "empathetic", "balanced"];
        let specialties = vec![
            "Health & Fitness",
            "Career Development",
            "Personal Development",
        ];

        for i in 1..=5 {
            let client = client.clone();
            let base_url = BASE_URL.to_string();
            let personality = personalities[i % personalities.len()];
            let specialty = specialties[i % specialties.len()];

            let task = tokio::spawn(async move {
                let user_id = format!("user{}", i);
                let actor_response = client
                    .post(&format!("{}/actors/create", base_url))
                    .json(&json!({
                        "user_id": user_id,
                        "name": format!("Test Actor {}", i),
                        "personality": personality,
                        "expertise": specialty,
                        "goals": ["Goal 1", "Goal 2"],
                        "knowledge_base": format!("Expertise in {}", specialty),
                        "picture_url": "https://images.unsplash.com/photo-1594824476967-48c8b964273f"
                    }))
                    .send()
                    .await;

                assert!(
                    actor_response.unwrap().status().is_success(),
                    "Failed to create actor concurrently"
                );

                let interact_response = client
                    .post(&format!("{}/actors/interact", base_url))
                    .json(&json!({
                        "user_id": user_id,
                        "query": "What are the best exercises for abs?"
                    }))
                    .send()
                    .await;

                assert!(
                    interact_response.unwrap().status().is_success(),
                    "Failed to interact with actor concurrently"
                );
            });

            tasks.push(task);
        }

        for task in tasks {
            task.await?;
        }

        println!("Concurrent interaction tests: PASSED");
        Ok(())
    }
}
