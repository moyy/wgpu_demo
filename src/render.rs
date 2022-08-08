use std::{sync::Arc, thread, time::Duration};

use pi_async::rt::{worker_thread::WorkerRuntime, AsyncRuntime, AsyncRuntimeBuilder};

use crate::app;

lazy_static! {
    // 渲染运行时：录制 渲染 指令 + raf 调度
    pub(crate) static ref RENDER_RUNTIME: WorkerRuntime = AsyncRuntimeBuilder::default_worker_thread(Some("render_runtime"), None, None, None);
}

static mut RENDER_CONTEXT: Option<RenderContext> = None;

// ============= 公共方法
pub fn render_frame() {
    let _ = RENDER_RUNTIME.spawn(RENDER_RUNTIME.alloc(), async move {
        // TODO 之后 改成 presentation...get_current_texture
        thread::sleep(Duration::from_micros(1));
        app::spawn_animation_frame();
    });
}

// ============= crate 公共方法

pub(crate) fn init_render(window: Arc<winit::window::Window>) {
    let _ = RENDER_RUNTIME.spawn(RENDER_RUNTIME.alloc(), async move {
        let render_context = RenderContext {};

        // TODO

        unsafe {
            RENDER_CONTEXT.replace(render_context);
        }
    });
}

// 渲染环境，封装
struct RenderContext {}

impl RenderContext {
    pub(crate) async fn set_pause(&self, is_pause: bool) {}

}
