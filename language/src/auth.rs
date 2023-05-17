//! Authentication related code.

use std::str::FromStr;

use tonic::Request;

use crate::{Credentials, Error};

/// API Key authorization
#[derive(Clone)]
pub struct APIKey {
    api_key: String,
}

/// Authentication interceptor
#[derive(Clone)]
pub enum Authentication {
    /// API Key authentication
    APIKey(APIKey),
    /// No authentication
    None,
}

impl Authentication {
    /// Build an authentication interceptor from the given credentials
    pub async fn build(credentials: Credentials) -> Result<Authentication, Error> {
        match credentials {
            Credentials::ApiKey(api_key) => {
                let authz = APIKey { api_key };

                Ok(Authentication::APIKey(authz))
            }
            Credentials::None => Ok(Authentication::None),
        }
    }
}

impl tonic::service::Interceptor for Authentication {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, tonic::Status> {
        match self {
            Authentication::APIKey(api_key_auth) => {
                let api_key = api_key_auth.api_key.clone();
                let api_key = FromStr::from_str(&api_key).unwrap();
                req.metadata_mut().insert("x-goog-api-key", api_key);
                Ok(req)
            }
            Authentication::None => Ok(req),
        }
    }
}
