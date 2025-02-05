use std::fmt;
use std::os::raw::{c_char, c_int};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::Layer;

// Android log priorities
const ANDROID_LOG_UNKNOWN: i32 = 0;
const ANDROID_LOG_DEFAULT: i32 = 1;
const ANDROID_LOG_VERBOSE: i32 = 2;
const ANDROID_LOG_DEBUG: i32 = 3;
const ANDROID_LOG_INFO: i32 = 4;
const ANDROID_LOG_WARN: i32 = 5;
const ANDROID_LOG_ERROR: i32 = 6;
const ANDROID_LOG_FATAL: i32 = 7;
const ANDROID_LOG_SILENT: i32 = 8;

#[link(name = "log")]
extern "C" {
    fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

pub struct AndroidLogger {
    tag: String,
}

impl AndroidLogger {
    pub fn new<S: Into<String>>(tag: S) -> Self {
        Self { tag: tag.into() }
    }

    fn convert_level(level: &Level) -> i32 {
        match *level {
            Level::TRACE => ANDROID_LOG_VERBOSE,
            Level::DEBUG => ANDROID_LOG_DEBUG,
            Level::INFO => ANDROID_LOG_INFO,
            Level::WARN => ANDROID_LOG_WARN,
            Level::ERROR => ANDROID_LOG_ERROR,
        }
    }
}

impl<S> Layer<S> for AndroidLogger
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        // 獲取事件級別並轉換為 Android 日誌優先級
        let metadata = event.metadata();
        let priority = Self::convert_level(metadata.level());

        // 格式化事件消息
        let mut message = String::new();
        let mut visitor = AndroidLogVisitor(&mut message);
        event.record(&mut visitor);

        // 轉換為 C 字符串並寫入 Android 日誌
        use std::ffi::CString;
        if let (Ok(tag), Ok(text)) = (
            CString::new(self.tag.as_str()),
            CString::new(message.as_str()),
        ) {
            unsafe {
                __android_log_write(priority, tag.as_ptr(), text.as_ptr());
            }
        }
    }
}

// 訪問者結構體用於收集事件字段
struct AndroidLogVisitor<'a>(&'a mut String);

impl<'a> tracing::field::Visit for AndroidLogVisitor<'a> {
    fn record_i64(&mut self, _field: &tracing::field::Field, value: i64) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}", value));
    }

    fn record_u64(&mut self, _field: &tracing::field::Field, value: u64) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}", value));
    }

    fn record_bool(&mut self, _field: &tracing::field::Field, value: bool) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}", value));
    }

    fn record_str(&mut self, _field: &tracing::field::Field, value: &str) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{}", value));
    }

    fn record_debug(&mut self, _field: &tracing::field::Field, value: &dyn fmt::Debug) {
        if !self.0.is_empty() {
            self.0.push_str(" ");
        }
        self.0.push_str(&format!("{:?}", value));
    }
}
