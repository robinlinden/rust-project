mod event;
mod event_loop;
mod key;
mod mouse_button;
mod window;

use event::*;
use event_loop::*;
use window::*;

fn main() {
    println!("Running event system");
    let mut system = EventLoop::new();
    let mut windows = vec![
        Window::builder(&mut system)
            .with_title("great window")
            .build(),
        Window::builder(&mut system)
            .with_title("bad window")
            .with_size(200, 200)
            .build(),
    ];

    loop {
        for event in system.poll_events() {
            match event {
                Event::Quit => return,
                Event::DestroyWindowRequest { id } => {
                    windows.retain(|window| *window != id);
                    if windows.is_empty() {
                        quit();
                    }
                }
                Event::MouseButtonDown { window, x, y, .. } => {
                    println!("Mouse lbutton down in {} at {} {}", window, x, y)
                }
                Event::MouseButtonUp { window, x, y, .. } => {
                    println!("Mouse lbutton up in {} at {} {}", window, x, y)
                }
                Event::MouseMove { window, x, y } => {
                    println!("Mouse move in {} at {} {}", window, x, y)
                }
                Event::KeyDown { window, key } => {
                    println!("Key down in {}: {:?}", window, key);
                }
                Event::KeyUp { window, key } => {
                    println!("Key up in {}: {:?}", window, key);
                }
            }
        }
        println!("Yielded!");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
