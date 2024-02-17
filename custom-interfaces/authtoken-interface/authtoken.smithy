// authtoken.smithy

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.kny.interfaces.authtoken", crate: "authtoken_interface" } ]

namespace org.kny.interfaces.authtoken

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U64

@wasmbus(
    contractId: "medio:interfaces:authtoken",
    actorReceive: true,
    providerReceive: true )
service Authtoken {
  version: "0.1",
  operations: [ CreateToken, VerifyToken ]
}

operation CreateToken {
  input: TokenConfig,
  output: String
}

operation VerifyToken {
  input: VerifyTokenRequest
  output: VerifyTokenResponse
}

structure TokenConfig {
  @required
  claims: Claims,
  @required
  secret: String
}

structure Claims {
  @required
  expires: U64
  @required
  uid: U64
}

structure VerifyTokenRequest {
  @required
  token: String,
  @required
  secret: String
}

structure VerifyTokenResponse {
  @required
  success: Boolean,
  reason: String,
  id: U64
}