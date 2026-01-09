use super::*;

// TODO: KeyEvent
// _ZN7android13InputConsumer18initializeKeyEventEPNS_8KeyEventEPKNS_12InputMessageE
// void InputConsumer::initializeKeyEvent(KeyEvent* event, const InputMessage* msg)

pub fn init_motion_event() {
    let motion_event_sym =
        "_ZN7android13InputConsumer21initializeMotionEventEPNS_11MotionEventEPKNS_12InputMessageE";
    let motion_event = match resolve_symbol("libinput.so", motion_event_sym) {
        Some(addr) if !addr.is_null() => addr,
        _ => {
            error!("{} not found", motion_event_sym);
            return;
        }
    };

    unsafe {
        let _ = ORIG_MOTION_EVENT.set(transmute(
            match hook(
                motion_event as Address,
                motion_event_hook as *const fn() as usize as Address,
            ) {
                Ok(addr) => addr,
                Err(_) => {
                    error!("Failed to hook {}", motion_event_sym);
                    return;
                }
            },
        ));
    }
}

static ORIG_MOTION_EVENT: OnceLock<fn(event: NonNull<AInputEvent>, msg: *const c_void) -> c_void> =
    OnceLock::new();
//https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_android.cpp
unsafe fn motion_event_hook(event: MotionEvent, msg: *const c_void) {
    ORIG_MOTION_EVENT.get().unwrap_unchecked()(event.ptr(), msg);

    if INIT.get() != Some(&true) {
        return;
    }

    let pointer_index = event.pointer_index();
    if pointer_index < event.pointer_count() {
        let event_pointer_index = event.pointer_at_index(pointer_index);

        let io = { &mut *addr_of_mut!(IMGUI_CONTEXT) }
            .as_mut()
            .unwrap_unchecked()
            .io_mut();

        match event.action() {
            MotionAction::Down | MotionAction::Up => {
                let tool_type = event_pointer_index.tool_type();
                if tool_type == ToolType::Finger || tool_type == ToolType::Unknown {
                    io.add_mouse_pos_event([event_pointer_index.x(), event_pointer_index.y()]);
                    io.add_mouse_button_event(
                        MouseButton::Left,
                        event.action() == MotionAction::Down,
                    );
                }
            }
            MotionAction::ButtonPress | MotionAction::ButtonRelease => {
                let button_state = event.button_state();
                io.add_mouse_button_event(imgui::MouseButton::Left, button_state.primary());
                io.add_mouse_button_event(imgui::MouseButton::Right, button_state.secondary());
                io.add_mouse_button_event(imgui::MouseButton::Middle, button_state.teriary());
            }
            MotionAction::Move | MotionAction::HoverMove => {
                io.add_mouse_pos_event([event_pointer_index.x(), event_pointer_index.y()]);
            }
            _ => {}
        }
    }
}
