use crate::{event::Event, event_loop::EventLoop, key::keycode_to_key, mouse_button::MouseButton};
use std::{ffi::CString, fmt, os::raw::c_int, ptr::null_mut};
use winapi::*;

#[derive(PartialEq, Eq)]
pub struct WindowId {
    hwnd: HWND,
}

pub struct Window {
    pub id: WindowId,
}

impl Window {
    pub fn builder(event_loop: &mut EventLoop) -> WindowBuilder {
        WindowBuilder::new(event_loop)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.id.hwnd);
        }
    }
}

impl PartialEq<WindowId> for Window {
    fn eq(&self, other: &WindowId) -> bool {
        self.id == *other
    }
}

impl fmt::Display for WindowId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hwnd as u64)
    }
}

extern "C" fn wnd_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let ev_loop = unsafe {
        let ev_loop: *mut EventLoop = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as _;
        &mut *ev_loop
    };
    match msg {
        WM_CLOSE => {
            ev_loop.post_event(Event::DestroyWindowRequest {
                id: WindowId { hwnd },
            });
            return 0;
        }
        WM_LBUTTONDOWN => {
            ev_loop.post_event(Event::MouseButtonDown {
                x: GET_X_LPARAM(l_param as u32) as f32,
                y: GET_Y_LPARAM(l_param as u32) as f32,
                button: MouseButton::Left,
                window: WindowId { hwnd },
            });
        }
        WM_LBUTTONUP => {
            ev_loop.post_event(Event::MouseButtonUp {
                x: GET_X_LPARAM(l_param as u32) as f32,
                y: GET_Y_LPARAM(l_param as u32) as f32,
                button: MouseButton::Left,
                window: WindowId { hwnd },
            });
        }
        WM_MOUSEMOVE => {
            ev_loop.post_event(Event::MouseMove {
                x: GET_X_LPARAM(l_param as u32) as f32,
                y: GET_Y_LPARAM(l_param as u32) as f32,
                window: WindowId { hwnd },
            });
        }
        WM_KEYDOWN => {
            ev_loop.post_event(Event::KeyDown {
                window: WindowId { hwnd },
                key: keycode_to_key(w_param as u8),
            });
        }
        WM_KEYUP => {
            ev_loop.post_event(Event::KeyUp {
                window: WindowId { hwnd },
                key: keycode_to_key(w_param as u8),
            });
        }
        WM_SETCURSOR | WM_NCHITTEST | WM_GETICON | WM_GETMINMAXINFO | WM_WINDOWPOSCHANGING
        | WM_NCCREATE | WM_NCMOUSEMOVE | WM_NCLBUTTONDOWN | WM_CAPTURECHANGED | WM_SYSCOMMAND
        | WM_CREATE | WM_NCCALCSIZE | WM_NCPAINT | WM_IME_SETCONTEXT | WM_IME_NOTIFY
        | WM_DESTROY | WM_KILLFOCUS | WM_ACTIVATEAPP | WM_ACTIVATE | WM_WINDOWPOSCHANGED
        | WM_NCDESTROY | WM_NCACTIVATE | WM_NCMOUSELEAVE | 0x90 | WM_SETFOCUS | WM_SIZE
        | WM_MOVE | WM_GETOBJECT => (),
        _ => println!("{} {} {} {}", hwnd as u64, msg, w_param, l_param),
    }
    unsafe { DefWindowProcA(hwnd, msg, w_param, l_param) }
}

pub struct WindowBuilder<'a> {
    event_loop: &'a mut EventLoop,
    title: CString,
    width: c_int,
    height: c_int,
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

    pub fn build(&mut self) -> Window {
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
            Window {
                id: WindowId { hwnd },
            }
        }
    }
}
