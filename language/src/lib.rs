//! The `gcp-vertex-ai-generative-ai-language` crate deals with the language
//! part of the Vertex AI Generative models.
//!
//! An async client library for GCP Vertex AI Generative models

pub mod auth;

pub use auth::Authentication;
use google::ai::generativelanguage::v1beta2::discuss_service_client::DiscussServiceClient;
use google::ai::generativelanguage::v1beta2::model_service_client::ModelServiceClient;
use google::ai::generativelanguage::v1beta2::text_service_client::TextServiceClient;
use tonic::codegen::http::uri::InvalidUri;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

/// Errors that can occur when using [LanguageClient].
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Transport error
    #[error("tonic transport error - {0}")]
    Tonic(#[from] tonic::transport::Error),
    /// Invalid URI.
    #[error("{0}")]
    InvalidUri(#[from] InvalidUri),
    /// Service error.
    #[error("Status: {}", .0.message())]
    Status(#[from] tonic::Status),
}

const CERTIFICATES: &str = include_str!("../certs/roots.pem");

/// Credentials to use to connect to the services
#[derive(Clone)]
pub enum Credentials {
    /// API Key - see https://cloud.google.com/docs/authentication/api-keys
    ApiKey(String),
    /// No authentication
    None,
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
    /// The Discuss service client. In particular, this client is used for
    /// [`DiscussServiceClient::count_message_tokens`] and
    /// [`DiscussServiceClient::generate_message`].
    pub discuss_service: DiscussServiceClient<
        tonic::service::interceptor::InterceptedService<Channel, Authentication>,
    >,
    /// The Model service client. Notably, this client is used for
    /// [`ModelServiceClient::list_models`] and
    /// [`ModelServiceClient::get_model`].
    pub model_service: ModelServiceClient<
        tonic::service::interceptor::InterceptedService<Channel, Authentication>,
    >,
    /// The Text service client. Notably, this client is used for
    /// [`TextServiceClient::generate_text`],
    /// and [`TextServiceClient::embed_text`].
    pub text_service:
        TextServiceClient<tonic::service::interceptor::InterceptedService<Channel, Authentication>>,
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

        let channel = Channel::from_shared(endpoint)?
            .user_agent("github.com/ssoudan/gcp-vertex-ai-generative-ai")?
            .tls_config(tls_config)?
            .connect_lazy();

        Self::from_channel(credentials, channel).await
    }

    /// Creates a new LanguageClient from a Channel.
    pub async fn from_channel(
        credentials: Credentials,
        channel: Channel,
    ) -> Result<LanguageClient, Error> {
        let discuss_service = {
            let auth = Authentication::build(credentials.clone()).await?;
            DiscussServiceClient::with_interceptor(channel.clone(), auth)
        };

        let model_service = {
            let auth = Authentication::build(credentials.clone()).await?;
            ModelServiceClient::with_interceptor(channel.clone(), auth)
        };

        let text_service = {
            let auth = Authentication::build(credentials).await?;
            TextServiceClient::with_interceptor(channel, auth)
        };

        Ok(Self {
            discuss_service,
            model_service,
            text_service,
        })
    }
}

#[cfg(test)]
mod test;

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
