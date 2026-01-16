use super::*;

// TODO: KeyEvent
// _ZN7android13InputConsumer18initializeKeyEventEPNS_8KeyEventEPKNS_12InputMessageE
// void InputConsumer::initializeKeyEvent(KeyEvent* event, const InputMessage* msg)

pub unsafe fn init_motion_event() {
    let motion_event_sym =
        "_ZN7android13InputConsumer21initializeMotionEventEPNS_11MotionEventEPKNS_12InputMessageE";
    match resolve_symbol("libinput.so", motion_event_sym) {
        Some(addr) if !addr.is_null() => {
            let _ = ORIG_MOTION_EVENT.set(transmute(
                match hook(
                    addr as Address,
                    motion_event_hook as *const fn() as usize as Address,
                ) {
                    Ok(addr) => addr,
                    Err(_) => {
                        error!("Failed to hook {}", motion_event_sym);
                        return;
                    }
                },
            ));
            return;
        }
        _ => {
            error!("{} not found", motion_event_sym);
        }
    };

    let consume_sym = "_ZN7android13InputConsumer7consumeEPNS_26InputEventFactoryInterfaceEblPjPPNS_10InputEventE";
    match resolve_symbol("libinput.so", consume_sym) {
        Some(addr) if !addr.is_null() => {
            let _ = ORIG_CONSUME.set(transmute(
                match hook(
                    addr as Address,
                    consume_hook as *const fn() as usize as Address,
                ) {
                    Ok(addr) => addr,
                    Err(_) => {
                        error!("Failed to hook {}", consume_sym);
                        return;
                    }
                },
            ));
        }
        _ => {
            error!("{} not found", consume_sym);
        }
    };
}

static ORIG_MOTION_EVENT: OnceLock<fn(event: NonNull<AInputEvent>, msg: *const c_void) -> c_void> =
    OnceLock::new();
//https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_android.cpp
unsafe fn motion_event_hook(event: MotionEvent, msg: *const c_void) {
    ORIG_MOTION_EVENT.get().unwrap_unchecked()(event.ptr(), msg);

    if INIT.get() != Some(&true) {
        return;
    }

    handle_motion_event(event);
}

static ORIG_CONSUME: OnceLock<
    unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        bool,
        i64,
        *mut u32,
        *mut *mut AInputEvent,
    ) -> i32,
> = OnceLock::new();
#[allow(non_snake_case)]
unsafe fn consume_hook(
    this: *mut c_void,
    factory: *mut c_void,
    consumeBatches: bool,
    frameTime: i64,
    outSeq: *mut u32,
    outEvent: *mut *mut AInputEvent,
) -> i32 {
    let result = ORIG_CONSUME.get().unwrap_unchecked()(
        this,
        factory,
        consumeBatches,
        frameTime,
        outSeq,
        outEvent,
    );

    if INIT.get() != Some(&true) || result != 0 || outEvent.is_null() || (*outEvent).is_null() {
        return result;
    }

    let event_ptr = std::ptr::NonNull::new(*outEvent);
    let event_type = AInputEvent_getType(*outEvent);
    if event_type != AINPUT_EVENT_TYPE_MOTION as i32 {
        return result;
    }

    if let Some(event_nn) = event_ptr {
        //let input_event = InputEvent::from_ptr(event_nn);
        let motion_event = MotionEvent::from_ptr(event_nn);
        handle_motion_event(motion_event);
    }

    result
}

unsafe fn handle_motion_event(event: MotionEvent) {
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
