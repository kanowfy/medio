metadata package = [ { namespace: "org.kny.medio.newsletter", crate: "newsletter_crate" } ]

namespace org.kny.medio.newsletter

use org.wasmcloud.model#wasmbus

/// Description of Newsletter service
@wasmbus( actorReceive: true )
service Newsletter {
  version: "0.1",
  operations: [ Subscribe ]
}

operation Subscribe {
  input: SubscribeRequest,
  output: SubscribeResponse
}

structure SubscribeRequest {
  @required
  email: String
}

structure SubscribeResponse {
  @required
  success: Boolean,
  reason: String
}
