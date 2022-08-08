//! Demo 框架
//!
//!     4个线程，3个单线程运行时
//!
//!     window 在主线程中，执行 窗口循环，遇到窗口事件，扔到 逻辑运行时 处理
//!     app （单线程）运行时，模拟 GUI逻辑 或 JS虚拟机，处理事件，进行 raf 回调
//!     render （单线程）运行时：模拟 执行 渲染图 + present + getCurrentTexture
//!     asset （单线程）运行时；封装：GPU 资源创建 成 async 函数
//!
#![feature(drain_filter)]

#[macro_use]
extern crate lazy_static;

pub mod app;
pub mod asset;
pub mod render;
mod window;

use std::sync::Arc;
use futures::future::BoxFuture;

/// 底层 线程 框架 的 入口
pub fn framework_main(log_level: &str, app_main_fn: BoxFuture<'static, ()>) {
    
    // windows，将 定时器 精度 提高到 1ms
    #[cfg(target_os = "windows")]
    unsafe {
        winapi::um::timeapi::timeBeginPeriod(1);
    }

    // 初始化 日志，设定 过滤等级
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .try_init();

    // 创建 窗口
    let (window, event_loop) = window::create_window("Example");
    let window = Arc::new(window);

    // 初始化 app
    app::App::init(window.clone(), app_main_fn);

    // 初始化 渲染
    // andoroid 的 实现，需要 在 窗口 Resume事件 中
    #[cfg(target_os = "windows")]
    render::init_render(window.clone());

    // 主线程 线程循环
    // 直到 event_loop 关闭 才会 退出
    window::run_window_loop(window, event_loop);
}