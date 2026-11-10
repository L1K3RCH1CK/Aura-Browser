//! Aura Browser — Rust + Tao + Wry Android shell.
//! The visible home screen is bundled in assets/index.html.

use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

const HOME: &str = include_str!("../assets/index.html");

// Wry requires generated Android JNI bindings and its Android setup hook.
// The cargo-mobile2 Wry scaffold supplies the matching WryActivity class.
#[cfg(target_os = "android")]
tao::android_binding!(
    com_aura,
    aura_browser,
    WryActivity,
    wry::android_setup,
    _start_app
);

#[cfg(target_os = "android")]
wry::android_binding!(com_aura, ttt);

#[cfg(target_os = "android")]
fn _start_app() {
    run();
}

#[cfg(not(target_os = "android"))]
pub fn run_desktop() {
    run();
}

fn run() {
    let event_loop = EventLoop::builder().build().expect("create event loop");
    let window = WindowBuilder::new()
        .with_title("Aura Browser")
        .with_resizable(true)
        .build(&event_loop)
        .expect("create window");

    let _webview = WebViewBuilder::new()
        .with_html(HOME)
        .with_devtools(cfg!(debug_assertions))
        .build(&window)
        .expect("create Aura WebView");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
