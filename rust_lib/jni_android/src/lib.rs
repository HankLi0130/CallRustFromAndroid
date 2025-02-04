// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

// https://docs.rs/jni/latest/jni/#the-rust-side
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_hankdev_app_android_RustLib_hello(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
) -> jstring {
    let name: String = env
        .get_string(&name)
        .expect("Couldn't get java string!")
        .into();

    let message = core::hello(&name);

    env.new_string(message)
        .expect("Couldn't get java string!")
        .into_raw()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_hankdev_app_android_RustLib_initLogging(_env: JNIEnv, _class: JClass) {
    core::init_logging()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_hankdev_app_android_RustLib_showLog(_env: JNIEnv, _class: JClass) {
    core::show_log()
}
