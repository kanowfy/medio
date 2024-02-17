// password.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.kny.interfaces.password", crate: "password_interface" } ]

namespace org.kny.interfaces.password

use org.wasmcloud.model#wasmbus

@wasmbus(
    contractId: "medio:interfaces:password",
    actorReceive: true,
    providerReceive: true )
service Password {
  version: "0.1",
  operations: [ HashPassword, VerifyPassword ]
}

operation HashPassword {
  input: HashPayload,
  output: String
}

operation VerifyPassword {
  input: VerifyPayload,
  output: Boolean
}

structure HashPayload {
  @required
  plain: String,
}

structure VerifyPayload {
  @required
  plain: String,
  @required
  hashed: String
}

