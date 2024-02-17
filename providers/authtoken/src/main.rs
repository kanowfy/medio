//! authtoken capability provider
//!
//!
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use wasmbus_rpc::provider::prelude::*;
use authtoken_interface::{Authtoken, AuthtokenReceiver, TokenConfig, VerifyTokenRequest, VerifyTokenResponse};

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(AuthtokenProvider::default(), Some("Authtoken".to_string()))?;

    eprintln!("authtoken provider exiting");
    Ok(())
}

/// authtoken capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Authtoken)]
struct AuthtokenProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for AuthtokenProvider {}
impl ProviderHandler for AuthtokenProvider {}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize
}

#[async_trait]
impl Authtoken for AuthtokenProvider {
    async fn create_token(&self, _ctx: &Context, req: &TokenConfig) -> RpcResult<String> {
        let mut timer = SystemTime::now();
        timer += Duration::from_secs(req.claims.expires);
        let exp = timer.duration_since(UNIX_EPOCH).unwrap().as_secs();

        let claims = Claims {
            sub: req.claims.uid.to_string(),
            role: "User".to_owned(),
            exp: exp as usize,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(req.secret.as_bytes())).map_err(|e| {
            RpcError::ActorHandler(format!("failed to create token: {}", e))
        })
    }

    async fn verify_token(&self, _ctx: &Context, req: &VerifyTokenRequest) -> RpcResult<VerifyTokenResponse> {
        match decode::<Claims>(
            &req.token,
            &DecodingKey::from_secret(req.secret.as_bytes()),
            &Validation::default()) {
                Ok(d) => Ok(VerifyTokenResponse {
                    success: true,
                    id: Some(d.claims.sub.parse().unwrap()),
                    reason: None
                }),
                Err(e) => Ok(VerifyTokenResponse {
                    success: false,
                    reason: Some(format!("failed to verify token: {}", e)),
                    id: None
                })
            }
    }
}