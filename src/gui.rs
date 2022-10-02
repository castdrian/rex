use crate::{fetch, images::fetch_image_bytes, response};
use eframe::egui;
use egui_extras::RetainedImage;
use voca_rs::*;
use poll_promise::Promise;

struct Resource {
    /// HTTP response
    response: ehttp::Response,
    /// If set, the response was an image.
    image: Option<RetainedImage>,
}

impl Resource {
    fn from_response(ctx: &egui::Context, response: ehttp::Response) -> Self {
        let content_type = response.content_type().unwrap_or_default();
        let image = if content_type.starts_with("image/") {
            RetainedImage::from_image_bytes(&response.url, &response.bytes).ok()
        } else {
            None
        };
        Self {
            response,
            image,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MyApp {
    search: String,
    description: String,
    species: String,
    sprite: RetainedImage,
    ptype: RetainedImage,
    stype: RetainedImage,
    abilities: String,
    dimensions: String,
    enabled: bool,
    shiny: bool,
    num: i64,
	#[cfg_attr(feature = "serde", serde(skip))]
    promise: Option<Promise<ehttp::Result<Resource>>>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search: "".to_owned(),
            description: "".to_owned(),
            species: "".to_owned(),
            sprite: RetainedImage::from_image_bytes("empty.png", include_bytes!("../assets/empty.png")).unwrap(),
            ptype: RetainedImage::from_image_bytes("empty.png", include_bytes!("../assets/empty.png")).unwrap(),
            stype: RetainedImage::from_image_bytes("empty.png", include_bytes!("../assets/empty.png")).unwrap(),
            abilities: "".to_owned(),
            dimensions: "".to_owned(),
            enabled: false,
            shiny: false,
            num: 0,
			promise: Default::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
			let empty_image = RetainedImage::from_image_bytes("empty.png", include_bytes!("../assets/empty.png")).unwrap();

            ui.horizontal(|ui| {
				let searchbox = ui.add(egui::TextEdit::singleline(&mut self.search)
				.hint_text("Pokémon | 000").desired_width(425.0));

				/* let ctx = ctx.clone();
                let (sender, promise) = Promise::new();
                let request = ehttp::Request::get(&EMPTY_IMAGE);
                ehttp::fetch(request, move |response| {
                    ctx.request_repaint(); // wake up UI thread
                    let resource = response.map(|response| Resource::from_response(&ctx, response));
                    sender.send(resource);
                });
                self.promise = Some(promise);

				if let Some(promise) = &self.promise {
					if let Some(result) = promise.ready() {
						match result {
							Ok(resource) => {
								ui_resource(ui, resource);
							}
							Err(error) => {
								// This should only happen if the fetch API isn't available or something similar.
								println!("Error: {}", error);
							}
						}
					} else {
						println!("Loading...");
					}
				} */

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
						self.description = mon.flavor_texts.get(0).unwrap().flavor.clone();
						self.sprite = RetainedImage::from_image_bytes(
							"sprite.png",
							&fetch_image_bytes(&format!("https://www.cpokemon.com/pokes/home/{}.png", mon.num)).unwrap(),
						).unwrap();
						self.ptype = RetainedImage::from_image_bytes(
							"ptype.jpg",
							&fetch_image_bytes(&format!("https://github.com/castdrian/pkmn-screens/raw/main/data/images/icons/types/{}.jpg", case::lower_case(mon.types.get(0).unwrap().primary.as_str()))).unwrap(),
						).unwrap();
						if mon.types.len() > 1 {
							self.stype = RetainedImage::from_image_bytes(
								"stype.jpg",
								&fetch_image_bytes(&format!("https://github.com/castdrian/pkmn-screens/raw/main/data/images/icons/types/{}.jpg", case::lower_case(mon.types.get(1).unwrap().secondary.as_str()))).unwrap(),
							).unwrap();
						} else {
							self.stype = empty_image;
						}
						self.abilities = format!("{}{}{}", mon.abilities.first.name, if mon.abilities.second.is_none() { format!("") } else { format!(" / {}", mon.abilities.second.as_ref().unwrap().name) }, if mon.abilities.hidden.is_none() { format!("") } else { format!(" | HA: {}", mon.abilities.hidden.as_ref().unwrap().name) }).to_owned();
						self.dimensions = format!("Height: {} M | Weight: {} KG", mon.height, mon.weight).to_owned();
						self.enabled = true;
						self.shiny = false;
						self.num = mon.num;
					} else {
						let query = fetch::name_query::Variables{
							pokemon: String::from(self.search.trim())
						};
						let response = fetch::fetch_dex_name(query).expect("Query unsuccessful!");
						let mon = response::gui_get_nameresult(response);

						self.species = format!("#{} {} | {}: {} {}: {}", mon.num, case::capitalize(&mon.species, true), "♂", mon.gender.male, "♀", mon.gender.female).to_owned();
						self.description = mon.flavor_texts.get(0).unwrap().flavor.clone();
						self.sprite = RetainedImage::from_image_bytes(
							"sprite.png",
							&fetch_image_bytes(&format!("https://www.cpokemon.com/pokes/home/{}.png", mon.num)).unwrap(),
						).unwrap();
						self.ptype = RetainedImage::from_image_bytes(
							"ptype.jpg",
							&fetch_image_bytes(&format!("https://github.com/castdrian/pkmn-screens/raw/main/data/images/icons/types/{}.jpg", case::lower_case(mon.types.get(0).unwrap().primary.as_str()))).unwrap(),
						).unwrap();
						if mon.types.len() > 1 {
							self.stype = RetainedImage::from_image_bytes(
								"stype.jpg",
								&fetch_image_bytes(&format!("https://github.com/castdrian/pkmn-screens/raw/main/data/images/icons/types/{}.jpg", case::lower_case(mon.types.get(1).unwrap().secondary.as_str()))).unwrap(),
							).unwrap();
						} else {
							self.stype = empty_image;
						}
						self.abilities = format!("{}{}{}", mon.abilities.first.name, if mon.abilities.second.is_none() { format!("") } else { format!(" / {}", mon.abilities.second.as_ref().unwrap().name) }, if mon.abilities.hidden.is_none() { format!("") } else { format!(" | HA: {}", mon.abilities.hidden.as_ref().unwrap().name) }).to_owned();
						self.dimensions = format!("Height: {} M | Weight: {} KG", mon.height, mon.weight).to_owned();
						self.enabled = true;
						self.shiny = false;
						self.num = mon.num;
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
			ui.horizontal(|ui| {
				ui.add_enabled_ui(self.enabled, |ui| {
					let button = ui.add(egui::ImageButton::new(
						self.sprite.texture_id(ctx),
						egui::vec2(128.0, 128.0),
					));

					if button.enabled() && button.hovered() {
						ctx.output().cursor_icon = egui::CursorIcon::PointingHand;
					}

					if button.clicked() {
						if self.shiny {
							self.sprite = RetainedImage::from_image_bytes(
								"sprite.png",
								&fetch_image_bytes(&format!("https://www.cpokemon.com/pokes/home/{}.png", self.num)).unwrap(),
							).unwrap();
							self.shiny = false;
						} else {
							self.sprite = RetainedImage::from_image_bytes(
								"sprite.png",
								&fetch_image_bytes(&format!("https://www.cpokemon.com/pokes/home/shiny/{}.png", self.num)).unwrap(),
							).unwrap();
							self.shiny = true;
						}
					}
				});
				ui.add(egui::Label::new(&self.description).wrap(true));
			});
			ui.add(egui::Label::new(""));
			ui.horizontal(|ui| {
                ui.label("Powered by:");
				ui.add(egui::Hyperlink::from_label_and_url("graphqlpokemon.favware.tech", "https://graphqlpokemon.favware.tech/v7"));
			});
        });
    }
}
