use user::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;

#[allow(dead_code)]
mod user;
mod store;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, User)]
struct UserActor {}

#[async_trait]
impl User for UserActor {
    async fn register(&self, ctx: &Context, input: &RegisterRequest) -> RpcResult<RegisterResponse> {
        info!("Received register request: {:?}", input);
        Ok(match store::register(ctx, input).await {
            Ok(()) => RegisterResponse {
                success: true,
                reason: None
            },

            Err(e) => RegisterResponse {
                success: false,
                reason: Some(format!("{}", e)),
            }
        })
    }
    async fn login(&self, ctx: &Context, input: &LoginRequest) -> RpcResult<LoginResponse> {
        Ok(match store::login(ctx, input).await {
            Ok(resp) => LoginResponse {
                success: true,
                token: Some(resp.token),
                user: Some(resp.user),
                reason: None
            },

            Err(e) => LoginResponse {
                success: false,
                reason: Some(format!("{}", e)),
                token: None,
                user: None
            }
        })

    }

    async fn get_user(&self, ctx: &Context, id: &u64) -> RpcResult<GetUserResponse> {
        Ok(match store::get_user_by_id(ctx, id).await {
            Ok(user) => GetUserResponse {
                success: true,
                user: Some(user),
                reason: None
            },

            Err(e) => GetUserResponse {
                success: false,
                reason: Some(format!("{}", e)),
                user: None
            }
        })

    }
}
