mod event_loop;
mod window;

use event_loop::*;

fn main() {
    println!("Running event system");
    let mut system = EventLoop::new();
    let mut running = true;
    while running {
        match system.poll_event() {
            Some(Event::QuitRequested) => {
                println!("Got quit!");
                running = false;
            }
            None => {
                println!("Yielded!");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
}
