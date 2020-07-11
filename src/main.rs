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

#[cfg(windows)]
fn yes_no_dialog(title: String, caption: String, yes_cb: impl Fn() -> (), no_cb: impl Fn() -> ()) {
    let title = CString::new(title.into_bytes()).unwrap();
    let caption = CString::new(caption.into_bytes()).unwrap();
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
fn yes_no_dialog(title: String, caption: String, yes_cb: impl Fn() -> (), no_cb: impl Fn() -> ()) {
    println!("{}", title.as_str());
    println!("{}", caption.as_str());

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let ans = input.chars().nth(0);
    if ans == Some('y') || ans == Some('Y') {
        yes_cb()
    } else {
        no_cb()
    }
}

fn main() {
    message_box(String::from("Great title"), String::from("Hello world"));
    yes_no_dialog(
        String::from("Do you like boxes?"),
        String::from("y/n?"),
        || println!("yes pressed"),
        || println!("no pressed"),
    );
}
