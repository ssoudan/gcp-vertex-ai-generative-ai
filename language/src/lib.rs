//! The `gcp-vertex-ai-generative-ai-language` crate deals with the language
//! part of the Vertex AI Generative models.
//!
//! An async client library for GCP Vertex AI Generative models

use std::str::FromStr;

use google::ai::generativelanguage::v1beta2::discuss_service_client::DiscussServiceClient;
use google::ai::generativelanguage::v1beta2::model_service_client::ModelServiceClient;
use google::ai::generativelanguage::v1beta2::text_service_client::TextServiceClient;
use tonic::codegen::http::uri::InvalidUri;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::Request;

/// Errors that can occur when using [LanguageClient].
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Transport error
    #[error("tonic transport error - {0}")]
    Tonic(#[from] tonic::transport::Error),
    /// Invalid URI.
    #[error("{0}")]
    InvalidUri(#[from] InvalidUri),
    /// Vizier service error.
    #[error("Status: {}", .0.message())]
    Status(#[from] tonic::Status),
}

const CERTIFICATES: &str = include_str!("../certs/roots.pem");

/// Credentials to use to connect to the services
#[derive(Clone)]
pub enum Credentials {
    /// API Key
    ApiKey(String),
}

/// google protos.
#[allow(missing_docs)]
pub mod google {

    /// google.api protos.
    pub mod api {

        include!(concat!(env!("OUT_DIR"), "/google.api.rs"));
    }

    /// google.ai protos.
    pub mod ai {

        /// google.ai.generativelanguage protos.
        pub mod generativelanguage {

            /// google.ai.generativelanguage.v1beta2 protos.
            pub mod v1beta2 {

                include!(concat!(
                    env!("OUT_DIR"),
                    "/google.ai.generativelanguage.v1beta2.rs"
                ));
            }
        }
    }
}

/// Generative Language client.
#[derive(Clone)]
pub struct LanguageClient {
    /// The Discuss service client.
    pub discuss_service:
        DiscussServiceClient<tonic::service::interceptor::InterceptedService<Channel, Authz>>,
    /// The Model service client.
    pub model_service:
        ModelServiceClient<tonic::service::interceptor::InterceptedService<Channel, Authz>>,
    /// The Text service client.
    pub text_service:
        TextServiceClient<tonic::service::interceptor::InterceptedService<Channel, Authz>>,
}

impl LanguageClient {
    /// Creates a new LanguageClient.
    ///
    /// # Example
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// use std::env;
    ///
    /// use gcp_vertex_ai_generative_language::LanguageClient;
    ///
    /// let creds = gcp_vertex_ai_generative_language::Credentials::ApiKey("my-api-key".to_string());
    ///
    /// let mut client = LanguageClient::new(creds).await.unwrap();
    ///
    /// # });
    /// ```
    pub async fn new(credentials: Credentials) -> Result<Self, Error> {
        let domain_name = "generativelanguage.googleapis.com".to_string();

        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(CERTIFICATES))
            .domain_name(&domain_name);

        let endpoint = format!("https://{endpoint}", endpoint = domain_name);

        dbg!(&endpoint);
        let channel = Channel::from_shared(endpoint)?
            .user_agent("github.com/ssoudan/gcp-vertex-ai-generative-ai")?
            .tls_config(tls_config)?
            .connect_lazy();

        let discuss_service = {
            let authz = Authz::build(credentials.clone()).await?;
            DiscussServiceClient::with_interceptor(channel.clone(), authz)
        };

        let model_service = {
            let authz = Authz::build(credentials.clone()).await?;
            ModelServiceClient::with_interceptor(channel.clone(), authz)
        };

        let text_service = {
            let authz = Authz::build(credentials).await?;
            TextServiceClient::with_interceptor(channel, authz)
        };

        Ok(Self {
            discuss_service,
            model_service,
            text_service,
        })
    }
}

/// API Key authorization
#[derive(Clone)]
pub struct APIKey {
    api_key: String,
}

/// Authorization interceptor
#[derive(Clone)]
pub enum Authz {
    /// API Key authorization
    APIKey(APIKey),
}

impl Authz {
    async fn build(credentials: Credentials) -> Result<Authz, Error> {
        match credentials {
            Credentials::ApiKey(api_key) => {
                let authz = APIKey { api_key };

                Ok(Authz::APIKey(authz))
            }
        }
    }
}

impl tonic::service::Interceptor for Authz {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, tonic::Status> {
        match self {
            Authz::APIKey(api_key_auth) => {
                let api_key = api_key_auth.api_key.clone();
                let api_key = FromStr::from_str(&api_key).unwrap();
                req.metadata_mut().insert("x-goog-api-key", api_key);
                Ok(req)
            }
        }
    }
}

#[cfg(test)]
mod tests {

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
}

#[cfg(test)]
mod common {
    use std::env;

    use crate::{Credentials, LanguageClient};

    pub(crate) async fn test_client() -> LanguageClient {
        let api_key = env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");

        LanguageClient::new(Credentials::ApiKey(api_key))
            .await
            .unwrap()
    }
}
