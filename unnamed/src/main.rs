mod event_loop;
mod window;

use event_loop::*;

fn main() {
    println!("Running event system");
    let mut system = EventLoop::new();
    let _win1 = system.window_builder().with_title("great window").build();
    let _win2 = system
        .window_builder()
        .with_title("bad window")
        .with_size(200, 200)
        .build();
    let mut windows = 2;
    loop {
        match system.poll_event() {
            Some(event) => match event {
                Event::Quit => break,
                Event::DestroyWindowRequest { id } => {
                    system.destroy_window(id);
                    windows -= 1;
                    if windows == 0 {
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
