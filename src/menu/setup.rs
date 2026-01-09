use super::*;

pub static INIT: OnceLock<bool> = OnceLock::new();
pub static mut RENDERER: Option<AutoRenderer> = None;
pub static mut IMGUI_CONTEXT: Option<imgui::Context> = None;

pub unsafe fn setup() {
    let libegl = "libEGL.so";
    let sym = "eglSwapBuffers";

    loop {
        if ElfScanner::find(libegl).is_some() {
            break;
        }
        std::thread::sleep(Duration::from_millis(200));
    }

    let egl_swap_buffers = match resolve_symbol(libegl, sym) {
        Some(addr) if !addr.is_null() => addr,
        _ => {
            error!("{} not found", sym);
            return;
        }
    };

    let origin_egl = match hook(
        egl_swap_buffers as Address,
        swap_buffers_hook as *const fn() as usize as Address,
    ) {
        Ok(addr) => addr,
        Err(_) => {
            error!("Failed to hook {}", sym);
            return;
        }
    };
    let _ = ORIGIN_SWAP.set(transmute(origin_egl));

    input::init_motion_event();
}

pub unsafe fn setup_imgui() -> bool {
    let gl = glow::Context::from_loader_function(|s| egl::get_proc_address(s) as *const _);

    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);
    //imgui_context.style_mut().use_classic_colors();

    let io = imgui_context.io_mut();
    io.font_global_scale = 2.0;

    let _renderer = match AutoRenderer::new(gl, &mut imgui_context) {
        Ok(r) => r,
        Err(msg) => {
            error!("Failed to create renderer: {}", msg);
            return false;
        }
    };

    RENDERER = Some(_renderer);
    IMGUI_CONTEXT = Some(imgui_context);
    info!("initialized");
    true
}
