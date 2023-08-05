#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// https://webgpufundamentals.org/webgpu/lessons/webgpu-compute-shaders.html


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let native_options = eframe::NativeOptions{
        //depth_buffer: 8,
        //stencil_buffer: 8,
        //multisampling: 16,
        event_loop_builder: Some(Box::new(move |builder| {
        })),
        ..Default::default()
    };
    eframe::run_native(
        "f9",
        native_options,
        Box::new(|cc: &eframe::CreationContext<'_>| Box::new(eframe_template::TemplateApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
