package hankdev.app.android

object RustLib {
    init {
        // The name of Rust library
        System.loadLibrary("jni_android")
    }

    external fun hello(name: String): String
}