#[cfg(windows)]
use {
    std::ffi::CString,
    std::os::raw::{c_char, c_int, c_uint, c_void},
    std::ptr::null_mut,
};

#[cfg(windows)]
type UINT = c_uint;
#[cfg(windows)]
type HWND = *mut c_void;
#[cfg(windows)]
type LPCSTR = *const c_char;

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    pub fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
}

#[cfg(windows)]
fn message_box(title: String, caption: String) {
    let title = CString::new(title.into_bytes()).unwrap();
    let caption = CString::new(caption.into_bytes()).unwrap();
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
fn message_box(title: String, caption: String) {
    println!("{}", title.as_str());
    println!("{}", caption.as_str());
}

fn main() {
    message_box(String::from("Great title"), String::from("Hello world"));
}
