//! 窗口创建 & 事件循环
//!
//! 窗口事件循环 运行在 主线程
//!
//! 收到消息，就往 logic 投递
//!
//!

use crate::app::App;
use std::sync::Arc;
use winit::{
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

// 创建 窗口
pub(crate) fn create_window(title: &str) -> (winit::window::Window, EventLoop<()>) {
    log::info!("Entering window::create_window...");

    let event_loop = EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title(title);

    let window = builder.build(&event_loop).unwrap();

    (window, event_loop)
}

// 运行 窗口循环
pub(crate) fn run_window_loop(window: Arc<winit::window::Window>, event_loop: EventLoop<()>) {
    log::info!("Entering window::run_window_loop...");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            event::Event::Suspended => {
                App::spawn_pause();
            }
            event::Event::Resumed => {
                App::spawn_resume();
            }
            event::Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            event::Event::WindowEvent {
                event:
                    WindowEvent::Resized(size)
                    | WindowEvent::ScaleFactorChanged {
                        new_inner_size: &mut size,
                        ..
                    },
                ..
            } => {
                App::spawn_resize(size.width, size.height);
            }
            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::R),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    App::spawn_exit();
                }
                _ => {}
            },

            event::Event::RedrawRequested(_) => {}

            _ => {}
        }
    });
}
