#![allow(bad_style)]

#[cfg(windows)]
use {
    std::ffi::CString,
    std::os::raw::{c_char, c_ushort, c_int, c_uint, c_void},
    std::ptr::null_mut,
};

#[cfg(target_pointer_width = "32")]
use {
    std::os::raw::{c_long},
};

#[cfg(windows)]
type UINT = c_uint;
#[cfg(windows)]
type HWND = *mut c_void;
#[cfg(windows)]
type LPCSTR = *const c_char;
#[cfg(windows)]
type WORD = c_ushort;
#[cfg(windows)]
type ATOM = WORD;

#[cfg(all(windows, target_pointer_width = "64"))]
type __u64 = u64;

#[cfg(all(windows, target_pointer_width = "32"))]
type UINT_PTR = c_uint;
#[cfg(all(windows, target_pointer_width = "64"))]
type UINT_PTR = __u64;

#[cfg(all(windows, target_pointer_width = "32"))]
type LONG_PTR = c_long;
#[cfg(all(windows, target_pointer_width = "64"))]
type LONG_PTR = __u64;

#[cfg(windows)]
type LPARAM = LONG_PTR;
#[cfg(windows)]
type WPARAM = UINT_PTR;
#[cfg(windows)]
type LRESULT = LONG_PTR;
#[cfg(windows)]
type PVOID = *mut c_void;
#[cfg(windows)]
type HANDLE = PVOID;
#[cfg(windows)]
type HINSTANCE = HANDLE;
#[cfg(windows)]
type HICON = HANDLE;
#[cfg(windows)]
type HBRUSH = HANDLE;
#[cfg(windows)]
type HCURSOR = HICON;
#[cfg(windows)]
type WNDPROC = extern fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;

#[cfg(windows)]
#[repr(C)]
struct tagWNDCLASSEXA {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCSTR,
    lpszClassName: LPCSTR,
    hIconSm: HICON,
}
#[cfg(windows)]
type WNDCLASSEXA = tagWNDCLASSEXA;

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
    fn RegisterClassExA(Arg1: *const WNDCLASSEXA) -> ATOM;
}

#[cfg(windows)]
extern fn wnd_proc(hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    println!("{} {} {} {}", hWnd as u64, msg, wParam, lParam);
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
    let ans = input.chars().nth(0);
    if ans == Some('y') || ans == Some('Y') {
        yes_cb()
    } else {
        no_cb()
    }
}
