#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use crate::{fetch, response};

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
			species: "".to_owned(),
			types: "".to_owned(),
			abilities: "".to_owned()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
				let searchbox = ui.add(egui::TextEdit::singleline(&mut self.search)
				.hint_text("Pokémon | 000").desired_width(425.0));

				if searchbox.lost_focus() && searchbox.ctx.input().key_pressed(egui::Key::Enter) {
					if self.search.trim().is_empty() {
						return;
					}
					if self.search.trim().parse::<i64>().is_ok() {
						let query = fetch::num_query::Variables{
							num: self.search.trim().parse::<i64>().unwrap()
						};
						let response = fetch::fetch_dex_num(query).expect("Query unsuccessful!");
						println!("Number: {}", self.search);
						println!("Result: {:?}", response);
						let mon = response::gui_get_numresult(response);
					} else {
						let query = fetch::name_query::Variables{
							pokemon: String::from(self.search.trim())
						};
						let response = fetch::fetch_dex_name(query).expect("Query unsuccessful!");
						println!("Name: {}", self.search);
						println!("Result: {:?}", response);
						let mon = response::gui_get_nameresult(response);
					}
					self.search = "".to_owned();
				}
            });
			ui.horizontal(|ui| {
                ui.label("Species: ");
                ui.label(&self.species);
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