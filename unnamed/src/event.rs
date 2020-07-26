use crate::mouse_button::MouseButton;
use crate::window::WindowId;

pub enum Event {
    DestroyWindowRequest {
        id: WindowId,
    },
    Quit,
    MouseButtonDown {
        x: f32,
        y: f32,
        button: MouseButton,
        window: WindowId,
    },
    MouseButtonUp {
        x: f32,
        y: f32,
        button: MouseButton,
        window: WindowId,
    },
    MouseMove {
        x: f32,
        y: f32,
        window: WindowId,
    },
}
