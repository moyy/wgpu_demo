## Android 构建

### 1、添加 构建目标

+ rustup target add aarch64-linux-android
+ rustup target add armv7-linux-androideabi

### 2、安装 

+ Java JDK 11，添加 环境变量 JAVA_HOME
+ Android SDK，添加 环境变量 NDK_HOME
+ NDK，添加 环境变量 ANDROID_SDK_ROOT
    - 截至 2022.08，[推荐 NDK 版本：23.1.7779620](https://github.com/rust-windowing/android-ndk-rs)
+ cargo install cargo-apk

### 3、使用 [android-ndk-rs](https://github.com/rust-windowing/android-ndk-rs)

#### src/lib.rs

``` rs
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    println!("Hello World");
}
```

#### Cargo.toml

``` toml

[lib]
crate-type = ["lib", "cdylib"]

[target.'cfg(target_os = "android")'.dependencies]
ndk-glue = "xxx" # Substitute this with the latest ndk-glue version you wish to use

```

#### 运行

+ usb 接上 手机
+ cargo apk run