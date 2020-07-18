use winapi;

#[cfg(windows)]
use {std::ffi::CString, std::ptr::null_mut, winapi::*};

#[cfg(windows)]
extern "C" fn wnd_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    println!("{} {} {} {}", h_wnd as u64, msg, w_param, l_param);
    if msg == WM_DESTROY {
        unsafe {
            PostQuitMessage(0);
        }
    }
    unsafe { DefWindowProcA(h_wnd, msg, w_param, l_param) }
}

pub enum Event {
    QuitRequested,
}

use std::collections::VecDeque;

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

fn make_test_window() {
    let window_class_name = CString::new("great window").unwrap();
    let window_class = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        style: 0,
        lpfnWndProc: wnd_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: null_mut(),
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
        lpszClassName: window_class_name.as_c_str().as_ptr(),
        hIconSm: null_mut(),
    };

    unsafe {
        RegisterClassExA(&window_class);
        let instance = GetModuleHandleA(null_mut());
        CreateWindowExA(
            WS_EX_APPWINDOW as DWORD,
            window_class_name.as_c_str().as_ptr(),
            window_class_name.as_c_str().as_ptr(),
            (WS_OVERLAPPEDWINDOW | WS_VISIBLE) as DWORD,
            0,
            0,
            640,
            480,
            null_mut(),
            null_mut(),
            instance,
            null_mut(),
        );
    }
}

#[cfg(windows)]
pub fn message_dialog(title: &str, caption: &str) {
    let title = CString::new(title.as_bytes()).unwrap();
    let caption = CString::new(caption.as_bytes()).unwrap();
    unsafe {
        MessageBoxA(
            null_mut(),
            caption.as_c_str().as_ptr(),
            title.as_c_str().as_ptr(),
            0,
        );
    }
}

#[cfg(not(windows))]
pub fn message_dialog(title: &str, caption: &str) {
    println!("{}", title);
    println!("{}", caption);
}

#[cfg(windows)]
pub fn yes_no_dialog(title: &str, caption: &str, yes_cb: impl Fn() -> (), no_cb: impl Fn() -> ()) {
    let title = CString::new(title.as_bytes()).unwrap();
    let caption = CString::new(caption.as_bytes()).unwrap();
    let res = unsafe {
        MessageBoxA(
            null_mut(),
            caption.as_c_str().as_ptr(),
            title.as_c_str().as_ptr(),
            4,
        )
    };

    if res == 7 {
        no_cb()
    } else if res == 6 {
        yes_cb()
    }
}

#[cfg(not(windows))]
pub fn yes_no_dialog(title: &str, caption: &str, yes_cb: impl Fn() -> (), no_cb: impl Fn() -> ()) {
    println!("{}", title);
    println!("{}", caption);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let ans = input.chars().next();
    if ans == Some('y') || ans == Some('Y') {
        yes_cb()
    } else {
        no_cb()
    }
}
