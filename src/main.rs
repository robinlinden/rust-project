use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr::null_mut;

type UINT = c_uint;
type HWND = *mut c_void;
type LPCSTR = *const c_char;

#[link(name = "user32")]
extern "system" {
    pub fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
}

fn main() {
    let text = CString::new("Hello, world").unwrap();
    let caption = CString::new("Great title").unwrap();
    unsafe {
        MessageBoxA(
            null_mut(),
            text.as_c_str().as_ptr(),
            caption.as_c_str().as_ptr(),
            0,
        );
    }
}
