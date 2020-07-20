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
        match system.poll_event() {
            Some(event) => match event {
                Event::Quit => break,
                Event::DestroyWindowRequest { id } => {
                    windows.remove(&id);
                    system.destroy_window(id);
                    if windows.is_empty() {
                        quit();
                    }
                }
            },

            None => {
                println!("Yielded!");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
}
