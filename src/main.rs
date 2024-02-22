#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let app = miti_tracefront::TraceFront::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Miti's TraceFront",
        native_options,
        Box::new(|_cc| Box::new(app))
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();
    let app = miti_tracefront::ExampleApp::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner
            ::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|_cc| Box::new(app))
            ).await
            .expect("failed to start eframe");
    });
}
