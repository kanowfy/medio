use std::collections::HashMap;

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::info;

#[allow(dead_code)]
mod article;
#[allow(dead_code)]
mod newsletter;
#[allow(dead_code)]
mod user;

mod handlers;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ApiGatewayActor {}

/// Implementation of the HttpServer capability contract
#[async_trait]
impl HttpServer for ApiGatewayActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches("/").split("/").skip(1).collect();
        match (req.method.as_ref(), segments.as_slice()) {
            ("GET", ["health"]) => {
                let response =
                    HashMap::from([("name", "Medio"), ("version", "0.1"), ("status", "healthy")]);

                HttpResponse::json(response, 200)
            },

            /*
            ("OPTIONS", _) => {
                info!("Received OPTIONS request {:?}", req.header);
                handlers::response("ok", 200)
            },
             */

            ("GET", ["articles"]) => handlers::get_all_article_handler(ctx).await,

            ("GET", ["articles", id]) => {
                handlers::get_one_article_handler(ctx, id.parse::<u64>().unwrap_or(0)).await
            },

            ("GET", ["author", id]) => handlers::authorize_article_handler(ctx, req, Some(id.parse::<u64>().unwrap_or(0))).await,

            | ("POST", ["articles"]) | ("PATCH", ["articles"]) | ("DELETE", ["articles"]) => {
                handlers::authorize_article_handler(ctx, req, None).await
            }

            ("POST", ["register"]) => match serde_json::from_slice(&req.body) {
                Ok(payload) => handlers::register_handler(ctx, payload).await,
                Err(e) => handlers::response(handlers::Message {
                    message: format!("malformed body: {:?}", e)
                }, 400)
            },

            ("POST", ["login"]) => match serde_json::from_slice(&req.body) {
                Ok(payload) => handlers::login_handler(ctx, payload).await,
                Err(e) => handlers::response(handlers::Message {
                    message: format!("malformed body: {:?}", e)
                }, 400)
            },

            ("POST", ["subscribe"]) => match serde_json::from_slice(&req.body) {
                Ok(payload) => handlers::subscribe_handler(ctx, payload).await,
                Err(e) => handlers::response(handlers::Message {
                    message: format!("malformed body: {:?}", e)
                }, 400)
            },

            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}
