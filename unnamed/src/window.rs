use crate::event_loop::{Event, EventLoop};
use std::{ffi::CString, ptr::null_mut};
use winapi::*;

extern "C" fn wnd_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    println!("{} {} {} {}", hwnd as u64, msg, w_param, l_param);
    if msg == WM_CLOSE {
        let ev_loop = unsafe {
            let ev_loop: *mut EventLoop = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as _;
            &mut *ev_loop
        };
        ev_loop.post_event(Event::DestroyWindowRequest {
            id: WindowId { hwnd },
        });
        return 0;
    }
    unsafe { DefWindowProcA(hwnd, msg, w_param, l_param) }
}

pub struct WindowBuilder<'a> {
    event_loop: &'a mut EventLoop,
    title: &'a str,
}

pub struct WindowId {
    hwnd: HWND,
}

impl WindowBuilder<'_> {
    pub fn new<'a>(event_loop: &'a mut EventLoop, title: &'a str) -> WindowBuilder<'a> {
        WindowBuilder { event_loop, title }
    }

    pub fn build(&mut self) -> WindowId {
        let window_class_name = CString::new(self.title).unwrap();
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
            let hwnd = CreateWindowExA(
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
            SetWindowLongPtrA(hwnd, GWLP_USERDATA, self.event_loop as *mut _ as LONG_PTR);
            WindowId { hwnd }
        }
    }
}

impl EventLoop {
    pub fn window_builder<'a>(&'a mut self, title: &'a str) -> WindowBuilder<'a> {
        WindowBuilder::new(self, title)
    }

    pub fn destroy_window(&self, id: WindowId) {
        unsafe {
            DestroyWindow(id.hwnd);
        }
    }
}
