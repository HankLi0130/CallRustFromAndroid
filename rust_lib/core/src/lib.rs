use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod tracing_layer_android;

pub fn hello(name: &str) -> String {
    format!("Hello, {}! This is from Rust.", name)
}

pub fn init_logging() {
    let android_layer = tracing_layer_android::AndroidNdkLayer::new("rust_lib core");

    tracing_subscriber::registry().with(android_layer).init();

    trace!("init done!");
    debug!("init done!");
    info!("init done!");
    warn!("init done!");
    error!("init done!");
}

pub fn show_log() {
    info!("show log from Rust!!!")
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn test_hello() {
        hello("world");
    }
}
