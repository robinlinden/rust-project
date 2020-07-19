mod event_loop;
mod window;

use event_loop::*;

fn main() {
    println!("Running event system");
    let mut system = EventLoop::new();
    system.make_test_window();
    loop {
        match system.poll_event() {
            Some(Event::QuitRequested) => {
                println!("Got quit!");
                quit();
            }
            Some(Event::Quit) => break,
            None => {
                println!("Yielded!");
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
}
