mod event_loop;
mod window;

use event_loop::*;
use std::collections::HashSet;

fn main() {
    println!("Running event system");
    let mut system = EventLoop::new();
    let mut windows = HashSet::new();
    windows.insert(system.new_window().with_title("great window").build());
    windows.insert(
        system
            .new_window()
            .with_title("bad window")
            .with_size(200, 200)
            .build(),
    );
    loop {
        for event in system.poll_events() {
            match event {
                Event::Quit => return,
                Event::DestroyWindowRequest { id } => {
                    windows.remove(&id);
                    EventLoop::destroy_window(&id);
                    if windows.is_empty() {
                        quit();
                    }
                }
                Event::MouseButtonDown { x, y, window, .. } => {
                    println!("Mouse lbutton down in {} at {} {}", window, x, y)
                }
                Event::MouseButtonUp { x, y, window, .. } => {
                    println!("Mouse lbutton up in {} at {} {}", window, x, y)
                }
                Event::MouseMove { x, y, window } => {
                    println!("Mouse move in {} at {} {}", window, x, y)
                }
            }
        }
        println!("Yielded!");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
