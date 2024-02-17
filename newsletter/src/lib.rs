use email_interface::{Email, EmailSender, MailConfig, MailContent, MailCredentials};
use newsletter::*;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_logging::info;
use wasmcloud_interface_messaging::{
    MessageSubscriber, MessageSubscriberReceiver, SubMessage,
};

use crate::article::ArticleDto;

#[allow(dead_code)]
mod article;
#[allow(dead_code)]
mod newsletter;
mod store;
mod template;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, MessageSubscriber, Newsletter)]
struct NewsletterActor {}

#[async_trait]
impl Newsletter for NewsletterActor {
    async fn subscribe(
        &self,
        ctx: &Context,
        req: &SubscribeRequest,
    ) -> RpcResult<SubscribeResponse> {
        info!("Received subscribe request: {:?}", req);
        Ok(match store::subscribe(ctx, req).await {
            Ok(()) => SubscribeResponse {
                success: true,
                reason: None,
            },
            Err(e) => SubscribeResponse {
                success: false,
                reason: Some(format!("{}", e).into()),
            },
        })
    }
}

#[async_trait]
impl MessageSubscriber for NewsletterActor {
    async fn handle_message(&self, ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
        info!("Received message: {:?}", msg);
        send_notification_email(ctx, msg).await
    }
}

async fn send_notification_email(ctx: &Context, msg: &SubMessage) -> RpcResult<()> {
    // get recipients
    info!("Fetching recipients");
    let recipients = match store::get_subscribers(ctx).await {
        Ok(rs) => rs,
        Err(e) => return Err(RpcError::ActorHandler(e.to_string())),
    };

    if recipients.is_empty() {
        return Ok(());
    }

    // parse article
    info!("Parsing particle from message");
    let article = bincode::deserialize::<ArticleDto>(&msg.body).unwrap();

    // construct body
    info!("Constructing mail body");
    let template = template::mail_template(&article, "http://localhost:5173");

    // send
    info!("Constructing mail config");
    let provider = EmailSender::new();
    let mail_config = MailConfig {
        message: MailContent {
            from: "Medio <nguyendung2002hl@gmail.com>".to_owned(),
            to: recipients,
            subject: format!("Medio: {}", article.title),
            is_html: true,
            body: template,
        },
        creds: MailCredentials {
            username: "nguyendung2002hl@gmail.com".to_owned(),
            password: "jwio cfev nrnk wjfo".to_owned(),
        },
        relay: "smtp.gmail.com".to_owned(),
    };
    info!("Sending email");
    match provider.send(ctx, &mail_config).await {
        Ok(res) => {
            if res.success {
                Ok(())
            } else {
                Err(RpcError::ActorHandler(format!(
                    "failed to send email: {}",
                    res.reason.unwrap()
                )))
            }
        }
        Err(e) => Err(e),
    }
}
