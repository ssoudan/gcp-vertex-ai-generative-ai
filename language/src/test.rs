use crate::common::test_client;
use crate::google::ai::generativelanguage::v1::part::Data;
use crate::google::ai::generativelanguage::v1::{
    Content, CountTokensRequest, EmbedContentRequest, GenerateContentRequest, GenerationConfig,
    ListModelsRequest, Part,
};

#[tokio::test]
async fn it_list_models() {
    let mut client = test_client().await;

    let req = ListModelsRequest {
        page_size: 3,
        page_token: "".to_string(),
    };

    dbg!(&req);

    let resp = client.model_service.list_models(req).await;

    dbg!(&resp);

    assert!(resp.is_ok());

    let resp = resp.unwrap();
    for m in resp.get_ref().models.iter() {
        println!("Model: {}: {}", m.name, m.description);
    }

    assert!(!resp.get_ref().models.is_empty());
}

#[tokio::test]
async fn it_count_tokens() {
    let mut client = test_client().await;

    let req = CountTokensRequest {
        model: "models/gemini-2.5-flash-lite".to_string(),
        contents: vec![Content {
            parts: vec![Part {
                data: Some(Data::Text("Hello, world!".to_string())),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };

    dbg!(&req);

    let resp = client.generative_service.count_tokens(req).await;

    dbg!(&resp);

    assert!(resp.is_ok());

    let resp = resp.unwrap();
    assert!(resp.get_ref().total_tokens > 0);
}

#[tokio::test]
async fn it_generates_text() {
    let mut client = test_client().await;

    let req = GenerateContentRequest {
        model: "models/gemini-2.5-flash-lite".to_string(),
        contents: vec![Content {
            parts: vec![Part {
                data: Some(Data::Text("Once upon a time,".to_string())),
                ..Default::default()
            }],
            ..Default::default()
        }],
        generation_config: Some(GenerationConfig {
            temperature: None,
            ..Default::default()
        }),
        ..Default::default()
    };

    dbg!(&req);

    let resp = client.generative_service.generate_content(req).await;

    dbg!(&resp);

    assert!(resp.is_ok());

    let resp = resp.unwrap();

    dbg!(resp);
}

#[tokio::test]
async fn it_embeds_text() {
    let mut client = test_client().await;

    let req = EmbedContentRequest {
        model: "models/text-embedding-004".to_string(),

        content: Some(Content {
            parts: vec![Part {
                data: Some(Data::Text("Je pense donc...".to_string())),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };

    dbg!(&req);

    let resp = client.generative_service.embed_content(req).await;

    dbg!(&resp);

    assert!(resp.is_ok());

    let resp = resp.unwrap();

    dbg!(resp);
}
