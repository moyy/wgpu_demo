use pi_async::rt::{AsyncRuntimeBuilder, worker_thread::WorkerRuntime, local_async_runtime};

lazy_static! {
    // 渲染资源 运行时，负责 创建 渲染 资源
    pub(crate) static ref RENDER_RES_RUNTIME: WorkerRuntime = AsyncRuntimeBuilder::default_worker_thread(Some("render_res_runtime"), None, None, None);
}