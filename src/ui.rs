#[cfg(windows)]
mod winapi {
    #![allow(bad_style)]

    use std::os::raw::{c_char, c_int, c_uint, c_ushort, c_void};

    #[cfg(target_pointer_width = "32")]
    use std::os::raw::c_long;

    pub type UINT = c_uint;
    pub type HWND = *mut c_void;
    pub type LPCSTR = *const c_char;
    pub type WORD = c_ushort;
    pub type ATOM = WORD;

    #[cfg(target_pointer_width = "64")]
    pub type __u64 = u64;

    #[cfg(target_pointer_width = "32")]
    pub type UINT_PTR = c_uint;
    #[cfg(target_pointer_width = "64")]
    pub type UINT_PTR = __u64;

    #[cfg(target_pointer_width = "32")]
    pub type LONG_PTR = c_long;
    #[cfg(target_pointer_width = "64")]
    pub type LONG_PTR = __u64;

    pub type LPARAM = LONG_PTR;
    pub type WPARAM = UINT_PTR;
    pub type LRESULT = LONG_PTR;
    pub type PVOID = *mut c_void;
    pub type HANDLE = PVOID;
    pub type HINSTANCE = HANDLE;
    pub type HICON = HANDLE;
    pub type HBRUSH = HANDLE;
    pub type HCURSOR = HICON;
    pub type WNDPROC = extern "C" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;

    #[repr(C)]
    pub struct tagWNDCLASSEXA {
        pub cbSize: UINT,
        pub style: UINT,
        pub lpfnWndProc: WNDPROC,
        pub cbClsExtra: c_int,
        pub cbWndExtra: c_int,
        pub hInstance: HINSTANCE,
        pub hIcon: HICON,
        pub hCursor: HCURSOR,
        pub hbrBackground: HBRUSH,
        pub lpszMenuName: LPCSTR,
        pub lpszClassName: LPCSTR,
        pub hIconSm: HICON,
    }
    pub type WNDCLASSEXA = tagWNDCLASSEXA;

    #[link(name = "user32")]
    extern "system" {
        pub fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
        pub fn RegisterClassExA(Arg1: *const WNDCLASSEXA) -> ATOM;
    }
}

#[cfg(windows)]
use {std::ffi::CString, std::ptr::null_mut, winapi::*};

#[cfg(windows)]
extern "C" fn wnd_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    println!("{} {} {} {}", h_wnd as u64, msg, w_param, l_param);
    0
}

#[cfg(windows)]
pub fn register_a_class() {
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
        lpszClassName: null_mut(),
        hIconSm: null_mut(),
    };

    unsafe {
        RegisterClassExA(&window_class);
        println!("Registered class!")
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
