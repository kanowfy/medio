//use std::{collections::HashMap, vec};

use authtoken_interface::{Authtoken, AuthtokenSender, VerifyTokenRequest};
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse};
use wasmcloud_interface_logging::info;

use crate::{
    article::{Article, ArticleSender, CreateArticleRequest, UpdateArticleRequest},
    newsletter::{Newsletter, NewsletterSender, SubscribeRequest},
    user::{LoginRequest, RegisterRequest, User, UserSender},
};

const ARTICLE_ACTOR: &str = "medio_articles";
const USER_ACTOR: &str = "medio_userauth";
const NEWSLETTER_ACTOR: &str = "medio_newsletters";

// temporary secret for testing
const JWT_SECRET: &str = "my secret";

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

pub(crate) async fn get_all_article_handler(ctx: &Context) -> RpcResult<HttpResponse> {
    let resp = ArticleSender::to_actor(ARTICLE_ACTOR)
        .get_all_articles(ctx)
        .await?;

    response(resp, 200)
}

pub(crate) async fn get_articles_for_author(ctx: &Context, id: u64) -> RpcResult<HttpResponse> {
    let resp = ArticleSender::to_actor(ARTICLE_ACTOR)
        .get_articles_for_author(ctx, &id)
        .await?;

    response(resp, 200)
}

pub(crate) async fn get_one_article_handler(ctx: &Context, id: u64) -> RpcResult<HttpResponse> {
    let resp = ArticleSender::to_actor(ARTICLE_ACTOR)
        .get_one_article(ctx, &id)
        .await?;

    if resp.success {
        response(resp, 200)
    } else {
        response(
            Message {
                message: resp.reason.unwrap(),
            },
            404,
        )
    }
}

pub(crate) async fn create_article_handler(
    ctx: &Context,
    payload: CreateArticleRequest,
) -> RpcResult<HttpResponse> {
    let resp = ArticleSender::to_actor(ARTICLE_ACTOR)
        .create_article(ctx, &payload)
        .await?;

    if resp.success {
        response(
            Message {
                message: "article created".to_owned(),
            },
            201,
        )
    } else {
        response(
            Message {
                message: resp.reason.unwrap(),
            },
            400,
        )
    }
}

pub(crate) async fn update_article_handler(
    ctx: &Context,
    payload: UpdateArticleRequest,
) -> RpcResult<HttpResponse> {
    let resp = ArticleSender::to_actor(ARTICLE_ACTOR)
        .update_article(ctx, &payload)
        .await?;
    if resp.success {
        response(
            Message {
                message: "article updated".to_owned(),
            },
            200,
        )
    } else {
        response(
            Message {
                message: resp.reason.unwrap(),
            },
            400,
        )
    }
}

pub(crate) async fn delete_article_handler(ctx: &Context, id: u64) -> RpcResult<HttpResponse> {
    let resp = ArticleSender::to_actor(ARTICLE_ACTOR)
        .delete_article(ctx, &id)
        .await?;

    if resp.success {
        response(
            Message {
                message: "article deleted".to_owned(),
            },
            200,
        )
    } else {
        response(
            Message {
                message: resp.reason.unwrap(),
            },
            400,
        )
    }
}

pub(crate) async fn register_handler(
    ctx: &Context,
    payload: RegisterRequest,
) -> RpcResult<HttpResponse> {
    let resp = UserSender::to_actor(USER_ACTOR)
        .register(ctx, &payload)
        .await?;

    if resp.success {
        response(resp, 201)
    } else {
        response(
            Message {
                message: resp.reason.unwrap(),
            },
            400,
        )
    }
}

pub(crate) async fn login_handler(ctx: &Context, payload: LoginRequest) -> RpcResult<HttpResponse> {
    let resp = UserSender::to_actor(USER_ACTOR)
        .login(ctx, &payload)
        .await?;

    if resp.success {
        response(resp, 200)
    } else {
        response(
            Message {
                message: resp.reason.unwrap(),
            },
            400,
        )
    }
}

pub(crate) async fn subscribe_handler(
    ctx: &Context,
    payload: SubscribeRequest,
) -> RpcResult<HttpResponse> {
    let resp = NewsletterSender::to_actor(NEWSLETTER_ACTOR)
        .subscribe(ctx, &payload)
        .await?;

    if resp.success {
        response(
            Message {
                message: "subscribed".to_owned(),
            },
            201,
        )
    } else {
        response(
            Message {
                message: resp.reason.unwrap(),
            },
            400,
        )
    }
}

pub(crate) async fn authorize_article_handler(
    ctx: &Context,
    req: &HttpRequest,
    params: Option<u64>,
) -> RpcResult<HttpResponse> {
    info!("Grabbing authorization header: {:?}", req.header);
    let values = req.header.get("authorization");
    if values.is_none() {
        return response(
            Message {
                message: "Missing authorization header".to_owned(),
            },
            403,
        );
    }

    let auth_header = match values.unwrap().into_iter().nth(0) {
        Some(h) => h,
        None => {
            return response(
                Message {
                    message: "Missing authorization header value".to_owned(),
                },
                403,
            )
        }
    };

    // check format
    info!("Verifying format of token {}", auth_header);
    let parts = auth_header.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 2 || !auth_header.starts_with("Bearer ") {
        return response(
            Message {
                message: "Invalid token format".to_owned(),
            },
            403,
        );
    }

    // verify
    let token = parts.into_iter().last().unwrap();
    info!("Verifying token {}", token);
    let provider = AuthtokenSender::new();
    let _ = match provider
        .verify_token(
            ctx,
            &VerifyTokenRequest {
                token: token.to_owned(),
                secret: JWT_SECRET.to_owned(),
            },
        )
        .await
    {
        Ok(resp) => {
            if resp.success {
                resp.id.unwrap()
            } else {
                return response(
                    Message {
                        message: "Invalid token".to_owned(),
                    },
                    403,
                );
            }
        }
        Err(e) => {
            return response(
                Message {
                    message: format!("Server error: {}", e),
                },
                500,
            )
        }
    };

    info!("Authorization successful, handling request");
    match req.method.as_str() {
        "GET" => get_articles_for_author(ctx, params.unwrap()).await,

        "POST" => match serde_json::from_slice(&req.body) {
            Ok(payload) => create_article_handler(ctx, payload).await,
            Err(e) => response(
                Message {
                    message: format!("malformed body: {:?}", e),
                },
                400,
            ),
        },
        "PATCH" => match serde_json::from_slice(&req.body) {
            Ok(payload) => update_article_handler(ctx, payload).await,
            Err(e) => response(
                Message {
                    message: format!("malformed body: {:?}", e),
                },
                400,
            ),
        },
        "DELETE" => match serde_json::from_slice(&req.body) {
            Ok(payload) => delete_article_handler(ctx, payload).await,
            Err(e) => response(
                Message {
                    message: format!("malformed body: {:?}", e),
                },
                400,
            ),
        },
        _ => response(
            Message {
                message: "not found".to_owned(),
            },
            404,
        ),
    }
}

pub(crate) fn response<T: Serialize>(msg: T, status: u16) -> RpcResult<HttpResponse> {
    /*HttpResponse::json_with_headers(
        msg,
        status,
        HashMap::from([
            ("Access-Control-Allow-Origin".to_owned(), vec!["*".to_owned()]),
            ("Access-Control-Allow-Methods".to_owned(), vec!["GET, POST, PATCH, DELETE, OPTIONS".to_owned()]),
            ("Access-Control-Allow-Headers".to_owned(), vec!["Origin, X-Requested-With, Accept, Authorization, Content-Type".to_owned()]),
            ("Access-Control-Allow-Credentials".to_owned(), vec!["true".to_owned()])
        ]),
    )
    */
    HttpResponse::json(msg, status)
}
