//! time capability provider
//!
//!
use wasmbus_rpc::{provider::prelude::*, Timestamp};
use time_interface::{Time,TimeReceiver};

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(TimeProvider::default(), Some("Time".to_string()))?;

    eprintln!("time provider exiting");
    Ok(())
}

/// time capability provider implementation
#[derive(Default, Clone, Provider)]
#[services(Time)]
struct TimeProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for TimeProvider {}
impl ProviderHandler for TimeProvider {}

#[async_trait]
impl Time for TimeProvider {
    async fn now(&self, _ctx: &Context) -> RpcResult<u64> {
        Ok(Timestamp::now().sec as u64)
    }
}
