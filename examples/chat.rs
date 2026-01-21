//! Example of using the generative_language client to chat.
//!
//! More examples in `gcp-vertex-ai-generative-language`.
use std::env;

use gcp_vertex_ai_generative_language::google::ai::generativelanguage::v1::part::Data;
use gcp_vertex_ai_generative_language::google::ai::generativelanguage::v1::{
    GenerateContentRequest, Part,
};
use gcp_vertex_ai_generative_language::{Credentials, LanguageClient};

#[tokio::main]
async fn main() {
    let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");

    let mut client = LanguageClient::new(Credentials::ApiKey(api_key))
        .await
        .unwrap();

    let req = GenerateContentRequest {
        model: "models/gemini-2.5-flash-lite".to_string(),
        contents: vec![
        gcp_vertex_ai_generative_language::google::ai::generativelanguage::v1::Content {
            parts: vec![
                Part {
                    data: Some(Data::Text(
                        "You are the young Bocuse, assisting a chef by providing detailed recipes and culinary advice."
                            .to_string(),
                    )),
                    ..Default::default()
                },
            ],
                ..Default::default()
        },
        gcp_vertex_ai_generative_language::google::ai::generativelanguage::v1::Content {
            parts: vec![
                Part {
                    data: Some(
                    Data::Text(
                        "It's late spring. I want to make an entremet and I'm looking for \
                    surprising pairings. I need suggestions for the base layer, a mousse, \
                    two different inserts and a coulis. Give me a 4 suggestions nicely formatted \
                    in a table."
                        .to_string(),
                    )),
                    ..Default::default()
                },],
                    role: "user".to_string(),
        ..Default::default()
        }],
        ..Default::default()
    };

    let resp = client.generative_service.generate_content(req).await;

    let resp = resp.unwrap();
    println!("Response:");
    for (i, m) in resp.get_ref().candidates.iter().enumerate() {
        if let Some(content) = m.content.as_ref() {
            println!(
                "({}) [{}]:\n{}",
                i,
                content.role,
                content
                    .parts
                    .iter()
                    .filter_map(|p| {
                        if let Some(Data::Text(t)) = p.data.as_ref() {
                            Some(t.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            println!("-----------------")
        }
    }
}
