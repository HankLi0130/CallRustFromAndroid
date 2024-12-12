# Call Rust from Kotlin on Android Client

## Key notes for Android

### Before setup

- Download NDK

### Setup

Add code below to project leve `build.gradle`

```
plugins {
    ...
    id "org.mozilla.rust-android-gradle.rust-android" version "0.9.5" apply false
}
```

Add code below to module level `build.gradle`

```
...

android {
    ...
    ndkVersion = '26.3.11579264'
    ...
}

...

// Apply the plugin
apply plugin: 'org.mozilla.rust-android-gradle.rust-android'

// Add cargo library config
cargo {
    pythonCommand = "/usr/bin/python3"

    // The path of the Rust module
    module  = "../../rust_android"
    
    // The name of the Rust library
    libname = "rust_android"
    
    targets = ["arm", "arm64", "x86", "x86_64"]
    verbose = true
}

// Add it as a dependency to one of your other build tasks, to build your rust code when you
// normally build your project
tasks.configureEach { task ->
    if ((task.name == 'javaPreCompileDebug' || task.name == 'javaPreCompileRelease')) {
        task.dependsOn 'cargoBuild'
    }
}
``` 

## References

- [Building cross platform library with Rust for ios and android](https://digvijayu.medium.com/building-cross-platform-library-with-rust-for-ios-and-android-c56a448e4804)
- [digvijayu / rust-crossplatform-library](https://github.com/digvijayu/rust-crossplatform-library)
- [mozilla / rust-android-gradle](https://github.com/mozilla/rust-android-gradle)