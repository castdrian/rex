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
        min_window_size: Some(egui::vec2(425.0, 290.0)),
        max_window_size: Some(egui::vec2(425.0, 290.0)),
        icon_data: Some(load_icon("././assets/rex.png")),
        ..Default::default()
    };

    eframe::run_native(
        "Rex - The Rust based PokéDex",
        options,
        Box::new(|_cc| Box::new(rex::MyApp::default())),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
pub fn main() {
	// Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();
	
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "rex_web",
        web_options,
        Box::new(|_cc| Box::new(rex::MyApp::default())),
    )
    .expect("failed to start eframe");
}