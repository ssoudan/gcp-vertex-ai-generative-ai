use crate::common::test_client;
use crate::google::ai::generativelanguage::v1beta2::{
    CountMessageTokensRequest, EmbedTextRequest, GenerateMessageRequest, GenerateTextRequest,
    ListModelsRequest, Message, MessagePrompt, TextPrompt,
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

    let req = CountMessageTokensRequest {
        model: "models/chat-bison-001".to_string(),
        prompt: Some(MessagePrompt {
            context: "Hello".to_string(),
            examples: vec![],
            messages: vec![Message {
                author: "1".to_string(),
                content: "How are you today?".to_string(),
                citation_metadata: None,
            }],
        }),
    };

    dbg!(&req);

    let resp = client.discuss_service.count_message_tokens(req).await;

    dbg!(&resp);

    assert!(resp.is_ok());

    let resp = resp.unwrap();
    assert!(resp.get_ref().token_count > 0);
}

#[tokio::test]
async fn it_generates_discussions() {
    let mut client = test_client().await;

    let req = GenerateMessageRequest {
        model: "models/chat-bison-001".to_string(),
        prompt: Some(MessagePrompt {
            context: "Hello".to_string(),
            examples: vec![],
            messages: vec![Message {
                author: "1".to_string(),
                content: "How are you today?".to_string(),
                citation_metadata: None,
            }],
        }),
        temperature: None,
        candidate_count: None,
        top_p: None,
        top_k: None,
    };

    dbg!(&req);

    let resp = client.discuss_service.generate_message(req).await;

    dbg!(&resp);

    assert!(resp.is_ok());

    let resp = resp.unwrap();

    dbg!(resp);
}

#[tokio::test]
async fn it_generates_text() {
    let mut client = test_client().await;

    let req = GenerateTextRequest {
        model: "models/text-bison-001".to_string(),
        prompt: Some(TextPrompt {
            text: "Hello my dear".to_string(),
        }),
        temperature: None,
        candidate_count: None,
        max_output_tokens: None,
        top_p: None,
        top_k: None,
        safety_settings: vec![],
        stop_sequences: vec![],
    };

    dbg!(&req);

    let resp = client.text_service.generate_text(req).await;

    dbg!(&resp);

    assert!(resp.is_ok());

    let resp = resp.unwrap();

    dbg!(resp);
}

#[tokio::test]
async fn it_embeds_text() {
    let mut client = test_client().await;

    let req = EmbedTextRequest {
        model: "models/embedding-gecko-001".to_string(),

        text: "Je pense donc...".to_string(),
    };

    dbg!(&req);

    let resp = client.text_service.embed_text(req).await;

    dbg!(&resp);

    assert!(resp.is_ok());

    let resp = resp.unwrap();

    dbg!(resp);
}
