use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod tracing_layer_android;

pub fn hello(name: &str) -> String {
    format!("Hello, {}! This is from Rust.", name)
}

pub fn init_logging() {
    #[cfg(target_os = "android")]
    {
        let layer = tracing_layer_android::AndroidLogger::new("rust_lib core");
        tracing_subscriber::registry().with(layer).init();
    }

    trace!("init done!");
    debug!("init done!");
    info!("init done!");
    warn!("init done!");
    error!("init done!");
}

pub fn show_log() {
    trace!("show log from Rust!!!");
    debug!("show log from Rust!!!");
    info!("show log from Rust!!!");
    warn!("show log from Rust!!!");
    error!("show log from Rust!!!");
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn test_hello() {
        hello("world");
    }
}
