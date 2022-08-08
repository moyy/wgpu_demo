use futures::future::BoxFuture;
use pi_async::rt::{worker_thread::WorkerRuntime, AsyncRuntime, AsyncRuntimeBuilder};
use pi_hash::XHashMap;
use pi_share::cell::TrustCell;
use std::sync::Arc;

lazy_static! {
    // 逻辑 运行时，模拟 js 虚拟机
    pub(crate) static ref APP_RUNTIME: WorkerRuntime = AsyncRuntimeBuilder::default_worker_thread(Some("app_runtime"), None, None, None);
}

static mut APP_CONTEXT: Option<App> = None;

// =========== 事件

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct AnimationFrameID(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct EventID(pub usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EventType {
    Pause,
    Resume,
    Exit,
    Resize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ResizeData {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Pause,
    Resume,
    Exit,
    Resize(ResizeData),
}

type TypeEvents = XHashMap<EventID, Arc<dyn Send + Sync + Fn(Event)>>;

// =========== App

// 为了 穿越 Future 做了这个设计
// `TODO` 以后看下 Tokio 有没有什么好的 设计手法；
#[derive(Clone)]
pub struct App(Arc<TrustCell<AppImpl>>);

struct AppImpl {
    // 窗口
    window: Arc<winit::window::Window>,

    // event
    current_event_id: usize,
    on_events: XHashMap<EventType, TypeEvents>,

    // raf
    current_raf_id: usize,
    // 用 option 是为了能换一个 新 的进去
    on_raf: Option<XHashMap<AnimationFrameID, Arc<dyn Send + Sync + Fn()>>>,
}

// =============== 公共方法
/// RAF
/// 只能 在 app 运行时 调用
pub fn request_animation_frame(cb: Arc<dyn Send + Sync + Fn()>) -> AnimationFrameID {
    let app = unsafe { APP_CONTEXT.as_ref().unwrap() };
    let id = app.get_next_animation_frame_id();

    let on_raf = &mut app.0.borrow_mut().on_raf;
    let on_raf = on_raf.as_mut().unwrap();

    on_raf.insert(id, cb);

    id
}

/// 立即：取消
/// 只能 在 app 运行时 调用
pub fn cancel_animation_frame(id: AnimationFrameID) {
    let app = unsafe { APP_CONTEXT.as_ref().unwrap() };

    let on_raf = &mut app.0.borrow_mut().on_raf;
    let on_raf = on_raf.as_mut().unwrap();

    on_raf.remove(&id);
}

/// 添加 事件监听器
/// 只能 在 app 运行时 调用
pub fn add_event_handler(r#type: EventType, cb: Arc<dyn Send + Sync + Fn(Event)>) -> EventID {
    let app = unsafe { APP_CONTEXT.as_ref().unwrap() };
    let id = app.get_next_event_id();

    let on_events = &mut app.0.borrow_mut().on_events;
    let on_events = on_events.entry(r#type).or_insert_with(XHashMap::default);

    on_events.insert(id, cb);

    id
}

/// 立即：取消
/// 只能 在 app 运行时 调用
pub fn remove_event_handler(r#type: EventType, id: EventID) {
    let app = unsafe { APP_CONTEXT.as_ref().unwrap() };

    let on_events = &mut app.0.borrow_mut().on_events;
    if let Some(on_events) = on_events.get_mut(&r#type) {
        on_events.remove(&id);
    }
}

impl App {
    #[inline]
    fn get_next_event_id(&self) -> EventID {
        let mut app = self.0.borrow_mut();
        app.current_event_id += 1;
        EventID(app.current_event_id)
    }

    #[inline]
    fn get_next_animation_frame_id(&self) -> AnimationFrameID {
        let mut app = self.0.borrow_mut();
        app.current_raf_id += 1;
        AnimationFrameID(app.current_raf_id)
    }

    pub(crate) fn init(window: Arc<winit::window::Window>, main_fn: BoxFuture<'static, ()>) {
        let app = AppImpl {
            window,

            current_raf_id: 0,
            current_event_id: 0,

            on_raf: Some(XHashMap::default()),
            on_events: XHashMap::default(),
        };

        let app = App(Arc::new(TrustCell::new(app)));
        unsafe {
            APP_CONTEXT.replace(app);
        }

        let _ = APP_RUNTIME.spawn(APP_RUNTIME.alloc(), main_fn);
    }
}

// ==================== `TODO` 之后会考虑 优先级 调度

// ==================== 以下方法 供 RENDER_RUNTIME 调用

// 告诉 app 可以 运行 下一帧 了
pub(crate) fn spawn_animation_frame() {
    let _ = APP_RUNTIME.spawn(APP_RUNTIME.alloc(), async move {
        let on_raf = {
            let app = unsafe { APP_CONTEXT.as_ref().unwrap() };

            let on_raf = &mut app.0.borrow_mut().on_raf;

            on_raf.replace(XHashMap::default()).unwrap()
        };

        for (_, f) in on_raf.into_iter() {
            f();
        }
    });
}

// ==================== 以下方法 供 主线程：窗口循环 调用

// 暂停
pub(crate) fn spawn_pause() {
    let _ = APP_RUNTIME.spawn(APP_RUNTIME.alloc(), async move {
        log::info!("app_runtime pause");

        let app = unsafe { APP_CONTEXT.as_ref().unwrap() };

        let on_events = &mut app.0.borrow_mut().on_events;
        if let Some(on_pauses) = on_events.get_mut(&EventType::Pause) {
            for (_, f) in on_pauses.iter() {
                f(Event::Pause);
            }
        }
    });
}

// 恢复
pub(crate) fn spawn_resume() {
    let _ = APP_RUNTIME.spawn(APP_RUNTIME.alloc(), async move {
        log::info!("app_runtime resume");

        let app = unsafe { APP_CONTEXT.as_ref().unwrap() };

        let on_events = &mut app.0.borrow_mut().on_events;
        if let Some(on_resumes) = on_events.get_mut(&EventType::Resume) {
            for (_, f) in on_resumes.iter() {
                f(Event::Pause);
            }
        }
    });
}

// 退出
pub(crate) fn spawn_exit() {
    let _ = APP_RUNTIME.spawn(APP_RUNTIME.alloc(), async move {
        log::info!("app_runtime exit");

        let app = unsafe { APP_CONTEXT.as_ref().unwrap() };

        let on_events = &mut app.0.borrow_mut().on_events;
        if let Some(on_exits) = on_events.get_mut(&EventType::Exit) {
            for (_, f) in on_exits.iter() {
                f(Event::Pause);
            }
        }
    });
}

// 重置大小
pub(crate) fn spawn_resize(width: u32, height: u32) {
    let _ = APP_RUNTIME.spawn(APP_RUNTIME.alloc(), async move {
        log::info!("app_runtime resize, width = {}, height = {}", width, height);

        let app = unsafe { APP_CONTEXT.as_ref().unwrap() };

        let on_events = &mut app.0.borrow_mut().on_events;
        if let Some(on_exits) = on_events.get_mut(&EventType::Resize) {
            for (_, f) in on_exits.iter() {
                f(Event::Resize(ResizeData { width, height }));
            }
        }
    });
}
