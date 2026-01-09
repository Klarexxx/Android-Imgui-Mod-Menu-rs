use super::*;

static mut SHOW_DEMO: bool = false; // or struct

pub unsafe fn draw(imgui: &mut imgui::Ui) {
    imgui
        .window("Hello, World!")
        .size([600.0, 500.0], imgui::Condition::FirstUseEver)
        .build(|| {
            imgui.checkbox("Show Demo", &mut *addr_of_mut!(SHOW_DEMO));
            imgui.separator();

            let mouse_pos = imgui.io().mouse_pos;
            imgui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });

    if SHOW_DEMO {
        imgui.show_demo_window(&mut *addr_of_mut!(SHOW_DEMO));
    }
}

pub unsafe fn render(width: EGLint, height: EGLint) {
    let (imgui, renderer) = (
        (*addr_of_mut!(IMGUI_CONTEXT)).as_mut().unwrap_unchecked(),
        (*addr_of_mut!(RENDERER)).as_mut().unwrap_unchecked(),
    );

    let io = imgui.io_mut();
    io.display_size = [width as f32, height as f32];

    let ui = imgui.new_frame();
    menu::draw(ui);
    renderer
        .render(imgui.render())
        .unwrap_or_else(|msg| error!("Error: {}", msg));
}

pub static ORIGIN_SWAP: OnceLock<fn(EGLDisplay, EGLSurface) -> EGLBoolean> = OnceLock::new();
pub unsafe fn swap_buffers_hook(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean {
    let mut w: EGLint = 0;
    let mut h: EGLint = 0;
    query_surface(dpy, surface, EGL_WIDTH, &mut w);
    query_surface(dpy, surface, EGL_HEIGHT, &mut h);

    let initialized = INIT.get_or_init(|| setup_imgui());

    if *initialized {
        menu::render(w, h);
    }

    ORIGIN_SWAP.get().unwrap_unchecked()(dpy, surface)
}
