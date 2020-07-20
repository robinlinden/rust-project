use crate::event_loop::{Event, EventLoop};
use std::{ffi::CString, os::raw::c_int, ptr::null_mut};
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
    title: CString,
    width: c_int,
    height: c_int,
}

pub struct WindowId {
    hwnd: HWND,
}

impl<'a> WindowBuilder<'a> {
    pub fn new(event_loop: &'a mut EventLoop) -> WindowBuilder<'a> {
        WindowBuilder {
            event_loop,
            title: CString::new("no name").unwrap(),
            width: 640,
            height: 480,
        }
    }

    pub fn with_title(&mut self, title: &str) -> &mut WindowBuilder<'a> {
        self.title = CString::new(title).unwrap();
        self
    }

    pub fn with_size(&mut self, width: c_int, height: c_int) -> &mut WindowBuilder<'a> {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build(&mut self) -> WindowId {
        let window_class_name = self.title.as_c_str().as_ptr();
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
            lpszClassName: window_class_name,
            hIconSm: null_mut(),
        };

        unsafe {
            RegisterClassExA(&window_class);
            let instance = GetModuleHandleA(null_mut());
            let hwnd = CreateWindowExA(
                WS_EX_APPWINDOW as DWORD,
                window_class_name,
                window_class_name,
                (WS_OVERLAPPEDWINDOW | WS_VISIBLE) as DWORD,
                0,
                0,
                self.width,
                self.height,
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
    pub fn new_window(&mut self) -> WindowBuilder {
        WindowBuilder::new(self)
    }

    pub fn destroy_window(&self, id: WindowId) {
        unsafe {
            DestroyWindow(id.hwnd);
        }
    }
}
