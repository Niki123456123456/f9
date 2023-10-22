#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// https://webgpufundamentals.org/webgpu/lessons/webgpu-compute-shaders.html

use log::log;
use log::Level;

// Icosphere Quadsphere UVsphere
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
   
    // https://github.com/Amanieu/parking_lot/issues/351
    env_logger::init();
    log!(Level::Error, "Test");
    let native_options = eframe::NativeOptions{
        depth_buffer: 32,
        stencil_buffer: 0,
        multisampling: 1,
        window_builder: Some(Box::new(|builder| {
            return builder.with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
        })),
        ..Default::default()
    };
    eframe::run_native(
        "f9",
        native_options,
        Box::new(|cc: &eframe::CreationContext<'_>| Box::new(f9::App::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions{
        depth_buffer: 32,
        ..Default::default()
    };

    log::info!("test");

    if cfg!(target_family = "wasm") {
        log::info!("wasm");
    }
    if cfg!(target_feature = "atomics") {
        log::info!("atomics");
    }
   

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Box::new(f9::App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
