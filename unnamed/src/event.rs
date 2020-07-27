use crate::{key::Key, mouse_button::MouseButton, window::WindowId};

pub enum Event {
    DestroyWindowRequest {
        id: WindowId,
    },
    Quit,
    MouseButtonDown {
        window: WindowId,
        button: MouseButton,
        x: f32,
        y: f32,
    },
    MouseButtonUp {
        window: WindowId,
        button: MouseButton,
        x: f32,
        y: f32,
    },
    MouseMove {
        window: WindowId,
        x: f32,
        y: f32,
    },
    KeyDown {
        window: WindowId,
        key: Key,
    },
    KeyUp {
        window: WindowId,
        key: Key,
    },
}
