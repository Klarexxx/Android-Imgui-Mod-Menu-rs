pub mod input;
pub mod menu;
pub mod setup;

pub use menu::*;
pub use setup::*;

use dobby_rs::{Address, hook, resolve_symbol};
use egl::*;
use imgui::*;
use imgui_glow_renderer::*;
use kittymemory::prelude::*;
use log::*;
use ndk::event::*;
use ndk_sys::AInputEvent;
use std::{ffi::c_void, mem::transmute, ptr::*, sync::OnceLock, time::Duration};
