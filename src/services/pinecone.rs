use reqwest::Client;
use serde_json::Value;

pub async fn init_pinecone() -> Result<(), String> {
    let pinecone_api_key = std::env::var("PINECONE_API_KEY")
        .map_err(|_| "PINECONE_API_KEY environment variable not set".to_string())?;
    let pinecone_index_url = std::env::var("PINECONE_INDEX_URL")
        .map_err(|_| "PINECONE_INDEX_URL environment variable not set".to_string())?;

    let client = Client::new();
    let response = client
        .get(&pinecone_index_url)
        .bearer_auth(&pinecone_api_key)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Pinecone: {}", e))?;

    if response.status().is_success() {
        println!("Connected to Pinecone successfully!");
        Ok(())
    } else {
        let error_body: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Pinecone error response: {}", e))?;
        Err(format!("Pinecone connection failed: {:?}", error_body))
    }
}

pub async fn test_pinecone_connection() -> Result<(), String> {
    let pinecone_api_key = std::env::var("PINECONE_API_KEY")
        .map_err(|_| "PINECONE_API_KEY environment variable not set".to_string())?;
    let pinecone_index_url = std::env::var("PINECONE_INDEX_URL")
        .map_err(|_| "PINECONE_INDEX_URL environment variable not set".to_string())?;

    let client = Client::new();
    let response = client
        .get(&format!("{}/describe_index_stats", pinecone_index_url))
        .bearer_auth(&pinecone_api_key)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Pinecone: {}", e))?;

    if response.status().is_success() {
        let stats: Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        println!("Index stats: {:?}", stats);
        Ok(())
    } else {
        Err(format!(
            "Failed to connect to Pinecone. Status: {}",
            response.status()
        ))
    }
}
