use std::sync::Arc;

use pi_async::rt::{worker_thread::WorkerRuntime, AsyncRuntimeBuilder};

lazy_static! {
    // 渲染运行时：录制 渲染 指令 + raf 调度
    pub(crate) static ref RENDER_RUNTIME: WorkerRuntime = AsyncRuntimeBuilder::default_worker_thread(Some("render_runtime"), None, None, None);
}

static mut RENDER_CONTEXT: Option<RenderContext> = None;

// 渲染环境，封装
pub(crate) struct RenderContext {

}

impl RenderContext {
    // 初始化
    pub(crate) async fn init(window: Arc<winit::window::Window>) {
        log::info!("RenderContext::init");
    }

    pub(crate) async fn set_pause(&self, is_pause: bool) {

    }

    // 由 app 的 raf 触发
    pub(crate) async fn handle_render(&self) {

    }
}