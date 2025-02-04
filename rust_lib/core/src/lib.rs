use crate::android_layer::AndroidNdkLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod android_layer;

pub fn hello(name: &str) -> String {
    format!("Hello, {}! This is from Rust.", name)
}

pub fn init_logging() {
    let android_layer = AndroidNdkLayer::new("rust_lib core");

    tracing_subscriber::registry()
        .with(android_layer)
        .init();

    info!("init done!");
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
