#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

pub fn main() {
	let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(425.0, 200.0)),
        max_window_size: Some(egui::vec2(425.0, 200.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rex - The Rust based PokéDex",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    search: String,
    species: String,
    types: String,
    abilities: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search: "".to_owned(),
			species: "Dragapult".to_owned(),
			types: "Ghost/Dragon".to_owned(),
			abilities: "Clear Body/Infiltrator".to_owned()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
				ui.add(egui::TextEdit::singleline(&mut self.search)
				.hint_text("Pokémon | 000").desired_width(150.0));

				if ui.button("Fetch Info!").clicked() {
					self.search = "Yveltal".to_owned();
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
            ui.label(format!("Input '{}'", self.species));
        });
    }
}