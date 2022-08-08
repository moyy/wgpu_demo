use futures::FutureExt;
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use wgpu_demo::{app, framework_main};

fn main() {
    framework_main(
        "info",
        async move {
            app_main().await;
        }
        .boxed(),
    );
}

async fn app_main() {
    app::add_event_handler(app::EventType::Pause, Arc::new(on_pause));

    app::add_event_handler(app::EventType::Resume, Arc::new(on_resume));

    app::add_event_handler(app::EventType::Exit, Arc::new(on_exit));

    app::add_event_handler(app::EventType::Resize, Arc::new(on_resize));

    app::request_animation_frame(Arc::new(on_raf));
}

fn on_pause(event: app::Event) {
    assert_eq!(event, app::Event::Pause);
    log::info!("example::window, enter on_pause...");
}

fn on_resume(event: app::Event) {
    assert_eq!(event, app::Event::Resume);
    log::info!("example::window, enter on_resume...");
}

fn on_resize(event: app::Event) {
    if let app::Event::Resize(app::ResizeData { width, height }) = event {
        log::info!(
            "example::window, enter on_resize, width = {}, height = {} ...",
            width,
            height
        );
    } else {
        panic!("example::window, enter on_resize, event not resize");
    }
}

fn on_exit(event: app::Event) {
    assert_eq!(event, app::Event::Exit);
}

fn on_raf() {
    let begin = Instant::now();
    thread::sleep(Duration::from_millis(1));
    let end = Instant::now();
    log::info!("example::window, enter on_raf, sleep: {:?}", end - begin);

    app::request_animation_frame(Arc::new(on_raf));
}