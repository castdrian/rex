#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

pub fn main() {
	let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rex - The Rust based PokéDex",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    mon: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            mon: "Dragapult".to_owned()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Search: ");
                ui.text_edit_singleline(&mut self.mon);
				if ui.button("Fetch Info!").clicked() {
					self.mon = "Yveltal".to_owned();
				}
            });
			ui.horizontal(|ui| {
                ui.label("Species: ");
            });
			ui.horizontal(|ui| {
                ui.label("Types: ");
            });
			ui.horizontal(|ui| {
                ui.label("Abilities: ");
            });
			ui.horizontal(|ui| {
                ui.label("Dimensions: ");
            });
            ui.label(format!("Input '{}'", self.mon));
        });
    }
}