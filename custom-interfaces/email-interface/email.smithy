// email.smithy
// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.kny.interfaces.email", crate: "email_interface" } ]

namespace org.kny.interfaces.email

use org.wasmcloud.model#wasmbus

@wasmbus(
    contractId: "medio:interfaces:email",
    actorReceive: true,
    providerReceive: true )
service Email {
  version: "0.1",
  operations: [ Send ]
}

operation Send {
  input: MailConfig,
  output: MailResult
}

structure MailConfig {
  @required
  message: MailContent,
  @required
  creds: MailCredentials,
  @required
  relay: String
}

structure MailContent {
  @required
  from: String,
  @required
  to: RecipientList,
  @required
  subject: String,
  @required
  isHtml: Boolean
  @required
  body: String
}

list RecipientList {
  member: String
}

structure MailCredentials {
  @required
  username: String
  @required
  password: String
}

structure MailResult {
  @required
  success: Boolean,
  reason: String
}