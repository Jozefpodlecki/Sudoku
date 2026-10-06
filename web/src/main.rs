// #![allow(unused_imports, dead_code, unused_variables, unused)]
#![no_std]
#![feature(const_trait_impl)]
#![feature(const_cmp)]

extern crate alloc;
extern crate std;

mod theme;
mod app;
mod utils;
mod routes;
mod components;
mod pages;
mod storage;
mod state;

use alloc::boxed::Box;
use wasm_logger::{init, Config};
use yew::Renderer;

use crate::app::{App, AppContext};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wasmalloc::WasmAlloc = wasmalloc::WasmAlloc::new();

fn main() -> Result<(), Box<dyn core::error::Error>> {
    
    let context = AppContext::new()?;

    let level = context
        .try_get_log_level_from_local_storage("RUST_LOG")
        .unwrap_or(if cfg!(debug_assertions) {
            log::Level::Trace
        } else {
            log::Level::Info
        });

    init(Config::new(level));

    console_error_panic_hook::set_once();

    let renderer = Renderer::<App>::with_root_and_props(context.body.clone().into(), context);
    renderer.render();

    Ok(())
}