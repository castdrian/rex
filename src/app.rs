use crate::{fetch, images::fetch_image_bytes, response};
use eframe::egui;
use egui_extras::RetainedImage;
use graphql_client::GraphQLQuery;
use std::sync::{Arc, Mutex};
use voca_rs::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/num_query.graphql",
    response_derives = "Debug, Clone"
)]
pub struct NumQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/name_query.graphql",
    response_derives = "Debug, Clone"
)]
pub struct NameQuery;

enum WebRequest {
    None,
    InProgress,
    Done(ehttp::Result<ehttp::Response>),
}

pub struct MyApp {
    search: String,
	num_mon: Option<num_query::NumQueryGetPokemonByDexNumber>,
	name_mon: Option<name_query::NameQueryGetFuzzyPokemon>,
    description: String,
    species: String,
    stored_sprite: Option<Vec<u8>>,
    stored_shiny_sprite: Option<RetainedImage>,
    stored_ptype: Option<RetainedImage>,
    stored_stype: Option<RetainedImage>,
    sprite: RetainedImage,
    ptype: RetainedImage,
    stype: RetainedImage,
    abilities: String,
    dimensions: String,
    enabled: bool,
    shiny: bool,
    num: i64,
    loading: bool,
    finished_num_fetch: bool,
	finished_name_fetch: bool,
	finished_sprite_fetch: bool,
	finished_shiny_sprite_fetch: bool,
	finished_ptype_fetch: bool,
	finished_stype_fetch: bool,
    num_web_req: Arc<Mutex<WebRequest>>,
    name_web_req: Arc<Mutex<WebRequest>>,
    ptype_web_req: Arc<Mutex<WebRequest>>,
    stype_web_req: Arc<Mutex<WebRequest>>,
    sprite_web_req: Arc<Mutex<WebRequest>>,
    shiny_sprite_web_req: Arc<Mutex<WebRequest>>,
	update_ui: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search: "".to_owned(),
			num_mon: None,
			name_mon: None,
            description: "".to_owned(),
            species: "".to_owned(),
            stored_sprite: None,
            stored_shiny_sprite: None,
			stored_ptype: None,
			stored_stype: None,
            sprite: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/empty.png"),
            )
            .unwrap(),
            ptype: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/empty.png"),
            )
            .unwrap(),
            stype: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/empty.png"),
            )
            .unwrap(),
            abilities: "".to_owned(),
            dimensions: "".to_owned(),
            enabled: false,
            shiny: false,
            num: 0,
            loading: false,
            finished_num_fetch: false,
			finished_name_fetch: false,
			finished_sprite_fetch: false,
			finished_shiny_sprite_fetch: false,
			finished_ptype_fetch: false,
			finished_stype_fetch: false,
            num_web_req: Arc::new(Mutex::new(WebRequest::None)),
			name_web_req: Arc::new(Mutex::new(WebRequest::None)),
			ptype_web_req: Arc::new(Mutex::new(WebRequest::None)),
			stype_web_req: Arc::new(Mutex::new(WebRequest::None)),
			sprite_web_req: Arc::new(Mutex::new(WebRequest::None)),
			shiny_sprite_web_req: Arc::new(Mutex::new(WebRequest::None)),
			update_ui: true,
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

				if searchbox.lost_focus() && searchbox.ctx.input().key_pressed(egui::Key::Enter) {
					if self.search.trim().is_empty() {
						return;
					}
					if self.search.trim().parse::<i64>().is_ok() {
						self.loading = true;
						self.finished_num_fetch = false;
						self.finished_name_fetch = false;
						self.finished_sprite_fetch = false;
						self.finished_shiny_sprite_fetch = false;
						self.finished_ptype_fetch = false;
						self.finished_stype_fetch = false;

						let query = num_query::Variables{
							num: self.search.trim().parse::<i64>().unwrap()
						};

						let num_request_body = NumQuery::build_query(query);
						let num_request_body = serde_json::to_vec(&num_request_body);

						let num_request = ehttp::Request {
							headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
							..ehttp::Request::post(
								"https://graphqlpokemon.favware.tech/v7",
								num_request_body.unwrap(),
							)
						};

						let num_req_store = self.num_web_req.clone();
						*num_req_store.lock().unwrap() = WebRequest::InProgress;
						let ctx = ctx.clone();
						ehttp::fetch(num_request, move |response| {
							*num_req_store.lock().unwrap() = WebRequest::Done(response);
							ctx.request_repaint(); // Wake up UI thread
						});

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
				if self.loading {
					ui.spinner();
				}
			});
        });

		if self.finished_num_fetch == false {
			let num_fetch: &WebRequest = &self.num_web_req.lock().unwrap();

			if let WebRequest::InProgress = num_fetch {
				self.loading = true;
			}
			if let WebRequest::Done(response) = num_fetch {
				let body = serde_json::from_slice::<graphql_client::Response<num_query::ResponseData>>(
					&response.as_ref().unwrap().bytes,
				)
				.unwrap();
				let mon = body.data.unwrap().get_pokemon_by_dex_number;
				println!("{:?}", response.as_ref().unwrap().status);
				println!("{:?}", mon);
				self.num_mon = Some(mon.clone());
				self.num = mon.num;
				self.finished_num_fetch = true;

				// fetch sprites once the mon is fetched
				let ctx = ctx.clone();
				let sprite_request = ehttp::Request {
					headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "image/png")]),
					..ehttp::Request::get("https://dex.pkmn.dev/rex.png")
				};
				let sprite_req_store = self.sprite_web_req.clone();
					*sprite_req_store.lock().unwrap() = WebRequest::InProgress;
					ehttp::fetch(sprite_request, move |response| {
						*sprite_req_store.lock().unwrap() = WebRequest::Done(response);
						ctx.request_repaint();
					});
			}
		}
		// check if the sprite request is done
		if self.finished_sprite_fetch == false && self.finished_num_fetch == true {
			let sprite_fetch: &WebRequest = &self.sprite_web_req.lock().unwrap();

			if let WebRequest::InProgress = sprite_fetch {
				self.loading = true;
			}

			if let WebRequest::Done(response) = sprite_fetch {
				let bytes = response.as_ref().unwrap().bytes.to_vec();
				self.stored_sprite = Some(bytes);
				self.loading = false;
				self.finished_sprite_fetch = true;
			}
		}
		// update ui when num_mon and sprites are fetched
		if self.num_mon.is_some() && self.stored_sprite.is_some() && self.update_ui == true {
			let mon = self.num_mon.as_ref().unwrap();
			self.species = format!("#{} {} | {}: {} {}: {}", mon.num, case::capitalize(&mon.species, true), "♂", mon.gender.male, "♀", mon.gender.female).to_owned();
			self.description = mon.flavor_texts.get(0).unwrap().flavor.clone();
			self.sprite = RetainedImage::from_image_bytes("sprite.png", self.stored_sprite.as_ref().unwrap()).unwrap();
			/* self.ptype = RetainedImage::from_image_bytes(
				"ptype.jpg",
				std::fs::read(format!("./assets/{}.jpg", case::lower_case(mon.types.get(0).unwrap().primary.as_str())).to_string()).unwrap().as_slice(),
			).unwrap();
			if mon.types.len() > 1 {
				self.stype = RetainedImage::from_image_bytes(
					"stype.jpg",
					std::fs::read(format!("./assets/{}.jpg", case::lower_case(mon.types.get(1).unwrap().primary.as_str())).to_string()).unwrap().as_slice(),
				).unwrap();
			} else {
				self.stype = RetainedImage::from_image_bytes("empty.png", include_bytes!("../assets/empty.png")).unwrap();
			} */
			self.abilities = format!("{}{}{}", mon.abilities.first.name, if mon.abilities.second.is_none() { format!("") } else { format!(" / {}", mon.abilities.second.as_ref().unwrap().name) }, if mon.abilities.hidden.is_none() { format!("") } else { format!(" | HA: {}", mon.abilities.hidden.as_ref().unwrap().name) }).to_owned();
			self.dimensions = format!("Height: {} M | Weight: {} KG", mon.height, mon.weight).to_owned();
			self.enabled = true;
			self.shiny = false;
			self.loading = false;
			self.update_ui = false;
		}

		/* // fetch sprites when num mon is fetched and stored sprites are empty
		if self.num_mon.is_some() && self.stored_sprite.is_none() {
			let mon = self.num_mon.as_ref().unwrap();
			let sprite_request = ehttp::Request::get(format!("https://www.cpokemon.com/pokes/home/{}.png", mon.num));
			let sprite_req_store = self.num_web_req.clone();
				*sprite_req_store.lock().unwrap() = WebRequest::InProgress;
				ehttp::fetch(sprite_request, move |response| {
					*sprite_req_store.lock().unwrap() = WebRequest::Done(response);
				});

			let shiny_sprite_request = ehttp::Request::get(format!("https://www.cpokemon.com/pokes/home/shiny/{}.png", mon.num));
			let shiny_sprite_req_store = self.num_web_req.clone();
				*shiny_sprite_req_store.lock().unwrap() = WebRequest::InProgress;
				ehttp::fetch(shiny_sprite_request, move |response| {
					*shiny_sprite_req_store.lock().unwrap() = WebRequest::Done(response);
				});

		} */
    }
}
