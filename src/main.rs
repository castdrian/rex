#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    fn load_icon(path: &str) -> eframe::IconData {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(path)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        eframe::IconData {
            rgba: icon_rgba,
            width: icon_width,
            height: icon_height,
        }
    }

    let options = eframe::NativeOptions {
        initial_window_size: Some([425.0, 290.0].into()),
        min_window_size: Some([425.0, 290.0].into()),
        max_window_size: Some([425.0, 290.0].into()),
        resizable: false,
        icon_data: Some(load_icon("././assets/icons/rex.png")),
        ..Default::default()
    };

    eframe::run_native(
        "Rex - The Rust based PokéDex",
        options,
        Box::new(|_cc| Box::new(rex::MyApp::default())),
    );
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_wasm_hooks() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
pub fn main() {
    init_wasm_hooks();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "rex_web",
        web_options,
        Box::new(|_cc| Box::new(rex::MyApp::default())),
    )
    .expect("failed to start eframe");
}
