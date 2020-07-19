#![allow(bad_style)]

use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};

pub type UINT = c_uint;
pub type HWND = *mut c_void;
pub type LPCSTR = *const c_char;
pub type WORD = c_ushort;
pub type DWORD = c_ulong;
pub type ATOM = WORD;
pub type LONG = c_long;
pub type BOOL = c_int;

pub const TRUE: BOOL = 1;

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
pub type LPVOID = *mut c_void;
pub type HANDLE = PVOID;
pub type HINSTANCE = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HICON = HANDLE;
pub type HBRUSH = HANDLE;
pub type HMENU = HANDLE;
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

#[repr(C)]
pub struct tagPOINT {
    x: LONG,
    y: LONG,
}
pub type POINT = tagPOINT;

#[repr(C)]
pub struct tagMSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD,
}
pub type MSG = tagMSG;
pub type LPMSG = *mut tagMSG;

pub const WS_OVERLAPPED: c_long = 0x00000000;

pub const WS_VISIBLE: c_long = 0x10000000;
pub const WS_CAPTION: c_long = 0x00C00000;

pub const WS_SYSMENU: c_long = 0x00080000;
pub const WS_THICKFRAME: c_long = 0x00040000;

pub const WS_MINIMIZEBOX: c_long = 0x00020000;
pub const WS_MAXIMIZEBOX: c_long = 0x00010000;

pub const WS_EX_APPWINDOW: c_long = 0x00040000;

pub const WS_OVERLAPPEDWINDOW: c_long =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const WM_DESTROY: UINT = 0x0002;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUIT: UINT = 0x0012;

pub const PM_NOREMOVE: UINT = 0x0000;
pub const PM_REMOVE: UINT = 0x0001;

pub const GWLP_USERDATA: c_int = -21;

#[link(name = "user32")]
extern "system" {
    pub fn MessageBoxA(hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT) -> c_int;
    pub fn RegisterClassExA(Arg1: *const WNDCLASSEXA) -> ATOM;
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
    pub fn CreateWindowExA(
        dwExStyle: DWORD,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn GetMessageA(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn PeekMessageA(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn DispatchMessageA(lpMsg: *const MSG) -> LRESULT;
    pub fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn PostQuitMessage(nExitCode: c_int) -> c_void;
    pub fn SetWindowLongPtrA(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    pub fn GetWindowLongPtrA(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
}
