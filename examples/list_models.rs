//! Example of using the generative_language client to list models.
//!
//! More examples in `gcp-vertex-ai-generative-language`.
use std::env;

use gcp_vertex_ai_generative_language::google::ai::generativelanguage::v1beta2::ListModelsRequest;
use gcp_vertex_ai_generative_language::{Credentials, LanguageClient};

#[tokio::main]
async fn main() {
    let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");

    let mut client = LanguageClient::new(Credentials::ApiKey(api_key))
        .await
        .unwrap();

    let req = ListModelsRequest {
        page_size: 12,
        page_token: "".to_string(),
    };

    let resp = client.model_service.list_models(req).await;

    let resp = resp.unwrap();
    println!("Models:");
    for m in resp.get_ref().models.iter() {
        println!("- {}: {}", m.name, m.description);
    }

    if !resp.get_ref().next_page_token.is_empty() {
        println!(
            "There are more results. Next page token: {}",
            resp.get_ref().next_page_token
        );
    }
}
