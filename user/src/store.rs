use std::error::Error;
use authtoken_interface::{AuthtokenSender, Authtoken, TokenConfig, Claims};
use minicbor::{decode, Decode, Encode};
use password_interface::{PasswordSender, Password, HashPayload, VerifyPayload};
use time_interface::{TimeSender, Time};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::{error, info};
use wasmcloud_interface_sqldb::*;

use crate::user::{LoginRequest, RegisterRequest, UserDto};

const TABLE_NAME: &str = "users";
// temporary secret for testing, need env later
const JWT_SECRET: &str = "my secret";
const JWT_VALID_DURATION: u64 = 86400 * 3;

#[derive(Encode, Decode)]
struct UserModel {
    #[n(0)]
    id: u64,
    #[n(1)]
    username: String,
    #[n(2)]
    display_name: String,
    #[n(3)]
    password_hashed: String,
    #[n(4)]
    email: String,
    #[n(5)]
    created_at: u64,
}

pub struct AuthResponse {
    pub token: String,
    pub user: UserDto
}

pub(crate) async fn register(
    ctx: &Context,
    input: &RegisterRequest,
) -> Result<(), Box<dyn Error>> {
    info!("Hashing password");
    let hashed = PasswordSender::new().hash_password(ctx, &HashPayload {
        plain: input.password.clone()
    }).await?;

    let now = get_current_time(ctx).await?;

    info!("Creating user");
    let _resp = SqlDbSender::new().query(
        ctx,
        &format!("INSERT INTO {} (username, display_name, password_hashed, email, created_at) VALUES ('{}', '{}', '{}', '{}', {})",
        TABLE_NAME, input.username, input.display_name.clone().unwrap_or("".to_string()), hashed, input.email, now).into()
    ).await?;
    info!("Create user successful");

    Ok(())
}

pub(crate) async fn login(
    ctx: &Context,
    input: &LoginRequest,
) -> Result<AuthResponse, Box<dyn Error>> {
    // get user by username
    info!("retrieving user by username {}", &input.username);
    let user = match get_user_by_username(ctx, &input.username).await {
        Some(u) => u,
        None => {
            error!("user not found");
            return Err("user not found".into());
        }
    };

    // verify password
    info!("user found, verifying password...");
    
    if !PasswordSender::new().verify_password(ctx, &VerifyPayload {
        plain: input.password.clone(),
        hashed: user.password_hashed.clone()
    }).await? {
        error!("failed to verify password");
        return Err("incorrect password".into());
    }
     

    info!("valid password, constructing token...");
    // construct new token
    let token = generate_jwt(ctx, &user).await?;

    info!("login successfully");
    // return response
    Ok(AuthResponse {
        token,
        user: user.into(),
    })
}

pub(crate) async fn get_user_by_id(ctx: &Context, id: &u64) -> Result<UserDto, Box<dyn Error>> {
    info!("querying user with id {}", id);
    let resp = SqlDbSender::new().query(
        ctx,
        &format!(
            "SELECT id, username, display_name, password_hashed, email, created_at FROM {} WHERE id = {}",
            TABLE_NAME, id
        ).into()
    ).await?;

    if resp.num_rows == 0 {
        error!("no user with id {} found", id);
        return Err("user not found".into())
    }

    let mut rows: Vec<UserModel> = decode(&resp.rows)?;
    Ok(rows.remove(0).into())
}

async fn get_user_by_username(ctx: &Context, username: &str) -> Option<UserModel> {
    let resp = SqlDbSender::new().query(
        ctx,
        &format!(
            "SELECT id, username, display_name, password_hashed, email, created_at FROM {} WHERE username = '{}'",
            TABLE_NAME, username
        ).into()
    ).await.unwrap();

    if resp.num_rows == 0 {
        return None;
    }

    let mut rows: Vec<UserModel> = decode(&resp.rows).unwrap();

    Some(rows.remove(0))
}

async fn generate_jwt(ctx: &Context, user: &UserModel) -> Result<String, Box<dyn Error>> {
    let provider = AuthtokenSender::new();
    provider.create_token(ctx, &TokenConfig {
        claims: Claims {
            expires: JWT_VALID_DURATION,
            uid: user.id
        },
        secret: JWT_SECRET.to_owned()
    }).await.map_err(|e| e.into())
}

async fn get_current_time(ctx: &Context) -> RpcResult<u64> {
    let provider = TimeSender::new();
    provider.now(ctx).await
}

impl From<UserModel> for UserDto {
    fn from(u: UserModel) -> UserDto {
        UserDto {
            id: u.id,
            username: u.username,
            display_name: u.display_name,
            email: u.email,
            created_at: u.created_at,
        }
    }
}
