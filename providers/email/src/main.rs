//! email capability provider
//!
//!
use lettre::{Message, message::{header::{ContentType, self}, Mailboxes}, transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use wasmbus_rpc::provider::prelude::*;
use email_interface::{Email, EmailReceiver, MailResult, MailConfig};

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(EmailProvider::default(), Some("Email".to_string()))?;

    eprintln!("email provider exiting");
    Ok(())
}

/// email capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Email)]
struct EmailProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for EmailProvider {}
impl ProviderHandler for EmailProvider {}

#[async_trait]
impl Email for EmailProvider {
    async fn send(&self, _ctx: &Context, req: &MailConfig) -> RpcResult<MailResult> {
        let mailboxes: Mailboxes = build_recipient_string(req.message.to.clone()).parse().unwrap();
        let to_header: header::To = mailboxes.into();
        let email = Message::builder()
            .from(req.message.from.parse().unwrap())
            .mailbox(to_header)
            .subject(&req.message.subject)
            .header(if req.message.is_html { ContentType::TEXT_HTML } else { ContentType::TEXT_PLAIN})
            .body(req.message.body.clone())
            .unwrap();

        let creds = Credentials::new(req.creds.username.clone(), req.creds.password.clone());

        let mailer = SmtpTransport::relay(&req.relay)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => Ok(MailResult {
                success: true,
                reason: None
            }),
            Err(e) => Ok(MailResult {
                success: false,
                reason: Some(e.to_string())
            })
        }
    }
}

fn build_recipient_string(list: Vec<String>) -> String {
    list.into_iter()
        .map(|s| format!("<{}>", s))
        .collect::<Vec<String>>()
        .join(", ")
}