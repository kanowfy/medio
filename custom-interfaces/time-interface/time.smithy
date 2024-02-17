// time.smithy
// A simple service that calculates the factorial of a whole number


// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.kny.interfaces.time", crate: "time_interface" } ]

namespace org.kny.interfaces.time

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

@wasmbus(
    contractId: "medio:interfaces:time",
    actorReceive: true,
    providerReceive: true )
service Time {
  version: "0.1",
  operations: [ Now ]
}

/// Return current time as number of seconds since unix epoch
operation Now {
  output: U64
}

