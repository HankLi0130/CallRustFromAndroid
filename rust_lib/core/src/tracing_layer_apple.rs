use std::ffi::{c_char, CString};
use std::fmt;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::Layer;

// os_log_type_t constants
const OS_LOG_TYPE_DEFAULT: u8 = 0x00;
const OS_LOG_TYPE_INFO: u8 = 0x01;
const OS_LOG_TYPE_DEBUG: u8 = 0x02;
const OS_LOG_TYPE_ERROR: u8 = 0x10;
const OS_LOG_TYPE_FAULT: u8 = 0x11;

#[link(name = "System", kind = "framework")]
extern "C" {
    fn os_log_create(subsystem: *const c_char, category: *const c_char) -> *const std::ffi::c_void;
    fn os_log_with_type(
        log: *const std::ffi::c_void,
        log_type: u8,
        format: *const c_char,
        message: *const c_char,
    );
}

pub struct AppleLogger {
    os_log: *const std::ffi::c_void,
}

impl AppleLogger {
    pub fn new(subsystem: &str, category: &str) -> Self {
        let subsystem = CString::new(subsystem).unwrap();
        let category = CString::new(category).unwrap();

        let os_log = unsafe { os_log_create(subsystem.as_ptr(), category.as_ptr()) };

        Self { os_log }
    }

    fn convert_level(level: &Level) -> u8 {
        match *level {
            Level::TRACE => OS_LOG_TYPE_DEBUG,
            Level::DEBUG => OS_LOG_TYPE_DEBUG,
            Level::INFO => OS_LOG_TYPE_INFO,
            Level::WARN => OS_LOG_TYPE_DEFAULT,
            Level::ERROR => OS_LOG_TYPE_ERROR,
        }
    }
}

impl<S> Layer<S> for AppleLogger
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let metadata = event.metadata();
        let log_type = Self::convert_level(metadata.level());

        // 收集事件字段
        let mut message = String::new();
        let mut visitor = AppleLogVisitor(&mut message);
        event.record(&mut visitor);

        // 將消息發送到 os_log
        if let Ok(format_str) = CString::new("%s") {
            if let Ok(message) = CString::new(message) {
                unsafe {
                    os_log_with_type(self.os_log, log_type, format_str.as_ptr(), message.as_ptr());
                }
            }
        }
    }
}

// 訪問者結構體用於收集事件字段
struct AppleLogVisitor<'a>(&'a mut String);

impl<'a> tracing::field::Visit for AppleLogVisitor<'a> {
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}={}", field.name(), value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}={}", field.name(), value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}={}", field.name(), value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}=\"{}\"", field.name(), value));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}={:?}", field.name(), value));
    }
}
