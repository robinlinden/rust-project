use {crate::window::make_test_window, std::collections::VecDeque, std::ptr::null_mut, winapi::*};

pub enum Event {
    QuitRequested,
}

pub struct EventLoop {
    events: VecDeque<Event>,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        make_test_window();

        EventLoop {
            events: VecDeque::new(),
        }
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        if self.events.is_empty() {
            let mut msg = std::mem::MaybeUninit::<MSG>::uninit();
            unsafe {
                PeekMessageA(msg.as_mut_ptr(), null_mut(), 0, 0, PM_NOREMOVE);
                let mut msg = msg.assume_init();
                while PeekMessageA(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                    if msg.message == WM_QUIT {
                        self.events.push_back(Event::QuitRequested);
                    }
                }
            }
        }

        self.events.pop_front()
    }
}
