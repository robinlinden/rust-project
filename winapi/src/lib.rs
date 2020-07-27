#![allow(bad_style)]

use std::os::raw::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void};

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

pub const WM_CREATE: UINT = 0x0001;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_MOVE: UINT = 0x0003;
pub const WM_SIZE: UINT = 0x0005;
pub const WM_ACTIVATE: UINT = 0x0006;
pub const WM_SETFOCUS: UINT = 0x0007;
pub const WM_KILLFOCUS: UINT = 0x0008;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUIT: UINT = 0x0012;
pub const WM_ACTIVATEAPP: UINT = 0x001c;
pub const WM_SETCURSOR: UINT = 0x0020;
pub const WM_GETMINMAXINFO: UINT = 0x0024;
pub const WM_GETOBJECT: UINT = 0x003d;
pub const WM_WINDOWPOSCHANGING: UINT = 0x0046;
pub const WM_WINDOWPOSCHANGED: UINT = 0x0047;
pub const WM_GETICON: UINT = 0x07F;
pub const WM_NCCREATE: UINT = 0x0081;
pub const WM_NCDESTROY: UINT = 0x0082;
pub const WM_NCCALCSIZE: UINT = 0x0083;
pub const WM_NCHITTEST: UINT = 0x0084;
pub const WM_NCPAINT: UINT = 0x0085;
pub const WM_NCACTIVATE: UINT = 0x0086;
// pub const WM_UAHDESTROYWINDOW: UINT = 0x0090; // undocumented
pub const WM_NCMOUSEMOVE: UINT = 0x00A0;
pub const WM_NCLBUTTONDOWN: UINT = 0x00A1;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const WM_SYSCOMMAND: UINT = 0x0112;
pub const WM_MOUSEMOVE: UINT = 0x0200;
pub const WM_LBUTTONDOWN: UINT = 0x0201;
pub const WM_LBUTTONUP: UINT = 0x0202;
pub const WM_CAPTURECHANGED: UINT = 0x0215;
pub const WM_IME_SETCONTEXT: UINT = 0x0281;
pub const WM_IME_NOTIFY: UINT = 0x0282;
pub const WM_NCMOUSELEAVE: UINT = 0x02a2;

pub const PM_NOREMOVE: UINT = 0x0000;
pub const PM_REMOVE: UINT = 0x0001;

pub const GWLP_USERDATA: c_int = -21;

pub const VK_LBUTTON: c_uchar = 0x01;
pub const VK_RBUTTON: c_uchar = 0x02;
pub const VK_CANCEL: c_uchar = 0x03;
pub const VK_MBUTTON: c_uchar = 0x04;
pub const VK_XBUTTON1: c_uchar = 0x05;
pub const VK_XBUTTON2: c_uchar = 0x06;
pub const VK_BACK: c_uchar = 0x08;
pub const VK_TAB: c_uchar = 0x09;
pub const VK_CLEAR: c_uchar = 0x0C;
pub const VK_RETURN: c_uchar = 0x0D;
pub const VK_SHIFT: c_uchar = 0x10;
pub const VK_CONTROL: c_uchar = 0x11;
pub const VK_MENU: c_uchar = 0x12;
pub const VK_PAUSE: c_uchar = 0x13;
pub const VK_CAPITAL: c_uchar = 0x14;
pub const VK_KANA: c_uchar = 0x15;
pub const VK_HANGUEL: c_uchar = 0x15;
pub const VK_HANGUL: c_uchar = 0x15;
pub const VK_IME_ON: c_uchar = 0x16;
pub const VK_JUNJA: c_uchar = 0x17;
pub const VK_FINAL: c_uchar = 0x18;
pub const VK_HANJA: c_uchar = 0x19;
pub const VK_KANJI: c_uchar = 0x19;
pub const VK_IME_OFF: c_uchar = 0x1A;
pub const VK_ESCAPE: c_uchar = 0x1B;
pub const VK_CONVERT: c_uchar = 0x1C;
pub const VK_NONCONVERT: c_uchar = 0x1D;
pub const VK_ACCEPT: c_uchar = 0x1E;
pub const VK_MODECHANGE: c_uchar = 0x1F;
pub const VK_SPACE: c_uchar = 0x20;
pub const VK_PRIOR: c_uchar = 0x21;
pub const VK_NEXT: c_uchar = 0x22;
pub const VK_END: c_uchar = 0x23;
pub const VK_HOME: c_uchar = 0x24;
pub const VK_LEFT: c_uchar = 0x25;
pub const VK_UP: c_uchar = 0x26;
pub const VK_RIGHT: c_uchar = 0x27;
pub const VK_DOWN: c_uchar = 0x28;
pub const VK_SELECT: c_uchar = 0x29;
pub const VK_PRINT: c_uchar = 0x2A;
pub const VK_EXECUTE: c_uchar = 0x2B;
pub const VK_SNAPSHOT: c_uchar = 0x2C;
pub const VK_INSERT: c_uchar = 0x2D;
pub const VK_DELETE: c_uchar = 0x2E;
pub const VK_HELP: c_uchar = 0x2F;
pub const VK_LWIN: c_uchar = 0x5B;
pub const VK_RWIN: c_uchar = 0x5C;
pub const VK_APPS: c_uchar = 0x5D;
pub const VK_SLEEP: c_uchar = 0x5F;
pub const VK_NUMPAD0: c_uchar = 0x60;
pub const VK_NUMPAD1: c_uchar = 0x61;
pub const VK_NUMPAD2: c_uchar = 0x62;
pub const VK_NUMPAD3: c_uchar = 0x63;
pub const VK_NUMPAD4: c_uchar = 0x64;
pub const VK_NUMPAD5: c_uchar = 0x65;
pub const VK_NUMPAD6: c_uchar = 0x66;
pub const VK_NUMPAD7: c_uchar = 0x67;
pub const VK_NUMPAD8: c_uchar = 0x68;
pub const VK_NUMPAD9: c_uchar = 0x69;
pub const VK_MULTIPLY: c_uchar = 0x6A;
pub const VK_ADD: c_uchar = 0x6B;
pub const VK_SEPARATOR: c_uchar = 0x6C;
pub const VK_SUBTRACT: c_uchar = 0x6D;
pub const VK_DECIMAL: c_uchar = 0x6E;
pub const VK_DIVIDE: c_uchar = 0x6F;
pub const VK_F1: c_uchar = 0x70;
pub const VK_F2: c_uchar = 0x71;
pub const VK_F3: c_uchar = 0x72;
pub const VK_F4: c_uchar = 0x73;
pub const VK_F5: c_uchar = 0x74;
pub const VK_F6: c_uchar = 0x75;
pub const VK_F7: c_uchar = 0x76;
pub const VK_F8: c_uchar = 0x77;
pub const VK_F9: c_uchar = 0x78;
pub const VK_F10: c_uchar = 0x79;
pub const VK_F11: c_uchar = 0x7A;
pub const VK_F12: c_uchar = 0x7B;
pub const VK_F13: c_uchar = 0x7C;
pub const VK_F14: c_uchar = 0x7D;
pub const VK_F15: c_uchar = 0x7E;
pub const VK_F16: c_uchar = 0x7F;
pub const VK_F17: c_uchar = 0x80;
pub const VK_F18: c_uchar = 0x81;
pub const VK_F19: c_uchar = 0x82;
pub const VK_F20: c_uchar = 0x83;
pub const VK_F21: c_uchar = 0x84;
pub const VK_F22: c_uchar = 0x85;
pub const VK_F23: c_uchar = 0x86;
pub const VK_F24: c_uchar = 0x87;
pub const VK_NUMLOCK: c_uchar = 0x90;
pub const VK_SCROLL: c_uchar = 0x91;
pub const VK_LSHIFT: c_uchar = 0xA0;
pub const VK_RSHIFT: c_uchar = 0xA1;
pub const VK_LCONTROL: c_uchar = 0xA2;
pub const VK_RCONTROL: c_uchar = 0xA3;
pub const VK_LMENU: c_uchar = 0xA4;
pub const VK_RMENU: c_uchar = 0xA5;
pub const VK_BROWSER_BACK: c_uchar = 0xA6;
pub const VK_BROWSER_FORWARD: c_uchar = 0xA7;
pub const VK_BROWSER_REFRESH: c_uchar = 0xA8;
pub const VK_BROWSER_STOP: c_uchar = 0xA9;
pub const VK_BROWSER_SEARCH: c_uchar = 0xAA;
pub const VK_BROWSER_FAVORITES: c_uchar = 0xAB;
pub const VK_BROWSER_HOME: c_uchar = 0xAC;
pub const VK_VOLUME_MUTE: c_uchar = 0xAD;
pub const VK_VOLUME_DOWN: c_uchar = 0xAE;
pub const VK_VOLUME_UP: c_uchar = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: c_uchar = 0xB0;
pub const VK_MEDIA_PREV_TRACK: c_uchar = 0xB1;
pub const VK_MEDIA_STOP: c_uchar = 0xB2;
pub const VK_MEDIA_PLAY_PAUSE: c_uchar = 0xB3;
pub const VK_LAUNCH_MAIL: c_uchar = 0xB4;
pub const VK_LAUNCH_MEDIA_SELECT: c_uchar = 0xB5;
pub const VK_LAUNCH_APP1: c_uchar = 0xB6;
pub const VK_LAUNCH_APP2: c_uchar = 0xB7;
pub const VK_OEM_1: c_uchar = 0xBA;
pub const VK_OEM_PLUS: c_uchar = 0xBB;
pub const VK_OEM_COMMA: c_uchar = 0xBC;
pub const VK_OEM_MINUS: c_uchar = 0xBD;
pub const VK_OEM_PERIOD: c_uchar = 0xBE;
pub const VK_OEM_2: c_uchar = 0xBF;
pub const VK_OEM_3: c_uchar = 0xC0;
pub const VK_OEM_4: c_uchar = 0xDB;
pub const VK_OEM_5: c_uchar = 0xDC;
pub const VK_OEM_6: c_uchar = 0xDD;
pub const VK_OEM_7: c_uchar = 0xDE;
pub const VK_OEM_8: c_uchar = 0xDF;
pub const VK_OEM_102: c_uchar = 0xE2;
pub const VK_PROCESSKEY: c_uchar = 0xE5;
pub const VK_PACKET: c_uchar = 0xE7;
pub const VK_ATTN: c_uchar = 0xF6;
pub const VK_CRSEL: c_uchar = 0xF7;
pub const VK_EXSEL: c_uchar = 0xF8;
pub const VK_EREOF: c_uchar = 0xF9;
pub const VK_PLAY: c_uchar = 0xFA;
pub const VK_ZOOM: c_uchar = 0xFB;
pub const VK_NONAME: c_uchar = 0xFC;
pub const VK_PA1: c_uchar = 0xFD;
pub const VK_OEM_CLEAR: c_uchar = 0xFE;

pub fn LOWORD(l: DWORD) -> WORD {
    l as WORD
}

pub fn HIWORD(l: DWORD) -> WORD {
    ((l >> 16) & 0xFFFF) as WORD
}

#[test]
fn loword_returns_the_low_word() {
    let dword: DWORD = 0x12345678;
    let loword: WORD = LOWORD(dword);
    assert_eq!(loword, 0x5678);
}

#[test]
fn hiword_returns_the_high_word() {
    let dword: DWORD = 0x12345678;
    let loword: WORD = HIWORD(dword);
    assert_eq!(loword, 0x1234);
}

pub fn GET_Y_LPARAM(lp: DWORD) -> c_int {
    HIWORD(lp) as c_short as c_int
}

pub fn GET_X_LPARAM(lp: DWORD) -> c_int {
    LOWORD(lp) as c_short as c_int
}

#[test]
fn get_x_lparam_handles_negative_coordinates() {
    let dword: DWORD = 0x1234ffec;
    let loword: c_int = GET_X_LPARAM(dword);
    assert_eq!(loword, -20);
}

#[test]
fn get_y_lparam_handles_negative_coordinates() {
    let dword: DWORD = 0xffec5678;
    let loword: c_int = GET_Y_LPARAM(dword);
    assert_eq!(loword, -20);
}

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
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
}
