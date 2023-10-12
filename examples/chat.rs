//! Example of using the generative_language client to chat.
//!
//! More examples in `gcp-vertex-ai-generative-language`.
use std::env;

use gcp_vertex_ai_generative_language::google::ai::generativelanguage::v1beta2::{
    GenerateMessageRequest, Message, MessagePrompt,
};
use gcp_vertex_ai_generative_language::{Credentials, LanguageClient};

#[tokio::main]
async fn main() {
    let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");

    let mut client = LanguageClient::new(Credentials::ApiKey(api_key))
        .await
        .unwrap();

    let req = GenerateMessageRequest {
        model: "models/chat-bison-001".to_string(),
        prompt: Some(MessagePrompt {
            context: "You are the young Bocuse, assisting a chef by providing detailed recipes and culinary advice."
                .to_string(),
            examples: vec![],
            messages: vec![Message {
                author: "LeChef".to_string(), 
                content: "It's late spring. I want to make an entremet and I'm looking for \
                surprising pairings. I need suggestions for the base layer, a mousse, \
                two different inserts and a coulis. Give me a 4 suggestions nicely formatted \
                in a table."
                    .to_string(),
                citation_metadata: None,
            }],
        }),
        temperature: Some(0.8),
        candidate_count: Some(1),
        top_p: None,
        top_k: None,
    };

    let resp = client.discuss_service.generate_message(req).await;

    let resp = resp.unwrap();
    println!("Response:");
    for (i, m) in resp.get_ref().candidates.iter().enumerate() {
        println!("({}) [{}]:\n{}", i, m.author, m.content);
        println!("-----------------")
    }
}
