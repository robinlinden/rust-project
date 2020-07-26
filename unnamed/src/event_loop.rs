use crate::event::Event;
use winapi::*;
use {std::collections::VecDeque, std::ptr::null_mut};

pub fn quit() {
    unsafe {
        PostQuitMessage(0);
    }
}

pub struct EventLoop {
    events: VecDeque<Event>,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        EventLoop {
            events: VecDeque::new(),
        }
    }

    pub fn post_event(&mut self, ev: Event) {
        self.events.push_back(ev);
    }

    pub fn poll_events(&mut self) -> impl Iterator<Item = Event> + '_ {
        let mut msg = std::mem::MaybeUninit::<MSG>::uninit();
        unsafe {
            PeekMessageA(msg.as_mut_ptr(), null_mut(), 0, 0, PM_NOREMOVE);
            let mut msg = msg.assume_init();
            while PeekMessageA(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
                if let WM_QUIT = msg.message {
                    self.post_event(Event::Quit)
                }
            }
        }

        self.events.drain(..)
    }
}
