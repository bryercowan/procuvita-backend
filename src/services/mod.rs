pub mod pinecone;
pub mod supabase;

use supabase_rs::SupabaseClient;

pub fn init_supabase() -> Result<SupabaseClient, String> {
    supabase::SupabaseService::new().map(|service| service.get_client().clone())
}

pub async fn init_pinecone() -> Result<(), String> {
    pinecone::init_pinecone().await
}
