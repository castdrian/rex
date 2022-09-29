#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;
use voca_rs::*;
use crate::{fetch, response, images::{fetch_image_bytes}, constants::EMPTY_IMAGE};

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
	description: String,
    species: String,
    types: String,
	ptype: RetainedImage,
	stype: RetainedImage,
    abilities: String,
	dimensions: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search: "".to_owned(),
			description: "".to_owned(),
			species: "".to_owned(),
			types: "".to_owned(),
			ptype: RetainedImage::from_image_bytes(
                "blank.png",
                &fetch_image_bytes(EMPTY_IMAGE).unwrap(),
            )
            .unwrap(),
			stype: RetainedImage::from_image_bytes(
				"blank.png",
				&fetch_image_bytes(EMPTY_IMAGE).unwrap(),
			).unwrap(),
			abilities: "".to_owned(),
			dimensions: "".to_owned()
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
						let mon = response::gui_get_numresult(response);

						self.species = format!("#{} {} | {}: {} {}: {}", mon.num, case::capitalize(&mon.species, true), "♂", mon.gender.male, "♀", mon.gender.female).to_owned();
						self.ptype = RetainedImage::from_image_bytes(
							"ptype.jpg",
							&fetch_image_bytes(&format!("https://github.com/castdrian/pkmn-screens/raw/main/data/images/icons/types/{}.jpg", case::lower_case(mon.types.get(0).unwrap()))).unwrap(),
						).unwrap();
						if mon.types.len() > 1 {
							self.stype = RetainedImage::from_image_bytes(
								"stype.jpg",
								&fetch_image_bytes(&format!("https://github.com/castdrian/pkmn-screens/raw/main/data/images/icons/types/{}.jpg", case::lower_case(mon.types.get(1).unwrap()))).unwrap(),
							).unwrap();
						} else {
							self.stype = RetainedImage::from_image_bytes(
								"blank.png",
								&fetch_image_bytes(EMPTY_IMAGE).unwrap(),
							).unwrap();
						}
						self.abilities = format!("{}{}{}", mon.abilities.first, if mon.abilities.second == None { format!("") } else { format!(" / {}", mon.abilities.second.as_ref().unwrap()) }, if mon.abilities.hidden == None { format!("") } else { format!(" | HA: {}", mon.abilities.hidden.as_ref().unwrap()) }).to_owned();
						self.dimensions = format!("Height: {} M | Weight: {} KG", mon.height, mon.weight).to_owned();

					} else {
						let query = fetch::name_query::Variables{
							pokemon: String::from(self.search.trim())
						};
						let response = fetch::fetch_dex_name(query).expect("Query unsuccessful!");
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
				ui.label(&self.types);
				ui.add(egui::Image::new(self.ptype.texture_id(ctx), egui::vec2(50.0, 11.0)));
				ui.add(egui::Image::new(self.stype.texture_id(ctx), egui::vec2(50.0, 11.0)));
            });
			ui.horizontal(|ui| {
                ui.label("Abilities: ");
				ui.label(&self.abilities);
            });
			ui.horizontal(|ui| {
                ui.label("Dimensions: ");
				ui.label(&self.dimensions);
            });
			// add padding
			ui.add(egui::Label::new(""));
			ui.add(egui::Label::new(""));
			ui.add(egui::Label::new(""));
			ui.add(egui::Label::new(""));
			ui.horizontal(|ui| {
                ui.label("Powered by:");
				ui.add(egui::Hyperlink::from_label_and_url("graphqlpokemon.favware.tech", "https://graphqlpokemon.favware.tech/v7"));
			});
        });
    }
}