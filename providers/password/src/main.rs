//! password capability provider
//!
//!
use wasmbus_rpc::provider::prelude::*;
use password_interface::{Password, PasswordReceiver, VerifyPayload, HashPayload};

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(PasswordProvider::default(), Some("Password".to_string()))?;

    eprintln!("password provider exiting");
    Ok(())
}

/// password capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Password)]
struct PasswordProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for PasswordProvider {}
impl ProviderHandler for PasswordProvider {}

#[async_trait]
impl Password for PasswordProvider {
    async fn hash_password(&self, _ctx: &Context, req: &HashPayload) -> RpcResult<String> {
        match bcrypt::hash(&req.plain, bcrypt::DEFAULT_COST) {
            Ok(h) => Ok(h),
            Err(_) => Err(RpcError::Other("failed to hash password".to_string()))
        }
    }

    async fn verify_password(&self, _ctx: &Context, req: &VerifyPayload) -> RpcResult<bool> {
        match bcrypt::verify(&req.plain, &req.hashed) {
            Ok(b) => Ok(b),
            Err(_) => Err(RpcError::Other("failed to verify password".to_string()))
        }
    }
}