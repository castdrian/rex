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

struct Ablities {
    primary_name: String,
    primary_desc: String,
    secondary_name: Option<String>,
    secondary_desc: Option<String>,
    hidden_name: Option<String>,
    hidden_desc: Option<String>,
}

impl Default for Ablities {
    fn default() -> Self {
        Self {
            primary_name: String::new(),
            primary_desc: String::new(),
            secondary_name: None,
            secondary_desc: None,
            hidden_name: None,
            hidden_desc: None,
        }
    }
}

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
	gender_ratio: String,
    stored_sprite: Option<Vec<u8>>,
    stored_shiny_sprite: Option<Vec<u8>>,
    stored_ptype: Option<Vec<u8>>,
    stored_stype: Option<Vec<u8>>,
	ball: RetainedImage,
	smogon: RetainedImage,
	egg: RetainedImage,
    sprite: RetainedImage,
    ptype: RetainedImage,
    stype: RetainedImage,
    abilities: Ablities,
    dimensions: String,
    enabled: bool,
    shiny: bool,
    loading: bool,
    chose_num: bool,
    chose_name: bool,
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
			gender_ratio: "".to_owned(),
            stored_sprite: None,
            stored_shiny_sprite: None,
            stored_ptype: None,
            stored_stype: None,
            ball: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/icons/empty.png"),
            )
            .unwrap(),
            smogon: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/icons/empty.png"),
            )
            .unwrap(),
            egg: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/icons/empty.png"),
            )
            .unwrap(),
            sprite: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/icons/sprite.png"),
            )
            .unwrap(),
            ptype: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/icons/empty.png"),
            )
            .unwrap(),
            stype: RetainedImage::from_image_bytes(
                "empty.png",
                include_bytes!("../assets/icons/empty.png"),
            )
            .unwrap(),
            abilities: Ablities::default(),
            dimensions: "".to_owned(),
            enabled: false,
            shiny: false,
            loading: false,
            chose_num: false,
            chose_name: false,
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
            ui.horizontal(|ui| {
                let searchbox = ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("Pokémon | 000")
                        .desired_width(425.0),
                );

                if searchbox.lost_focus() && searchbox.ctx.input().key_pressed(egui::Key::Enter) {
                    if self.search.trim().is_empty() {
                        return;
                    }
                    if self.search.trim().parse::<i64>().is_ok() {
                        if self.search.trim().parse::<i64>().unwrap() < 1
                            || self.search.trim().parse::<i64>().unwrap() > 898
                        {
                            return;
                        }

                        self.loading = true;
                        self.update_ui = true;
                        self.chose_name = false;
                        self.finished_num_fetch = false;
                        self.finished_name_fetch = false;
                        self.finished_sprite_fetch = false;
                        self.finished_shiny_sprite_fetch = false;
                        self.finished_ptype_fetch = false;
                        self.finished_stype_fetch = false;
                        self.num_mon = None;
                        self.stored_sprite = None;
                        self.stored_shiny_sprite = None;
                        self.stored_ptype = None;
                        self.stored_stype = None;
                        self.num_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.name_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.ptype_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.stype_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.sprite_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.shiny_sprite_web_req = Arc::new(Mutex::new(WebRequest::None));

                        let query = num_query::Variables {
                            num: self.search.trim().parse::<i64>().unwrap(),
                        };

                        let num_request_body = NumQuery::build_query(query);
                        let num_request_body = serde_json::to_vec(&num_request_body);

                        let num_request = ehttp::Request {
                            headers: ehttp::headers(&[
                                ("Accept", "*/*"),
                                ("Content-Type", "application/json"),
                            ]),
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
                        self.chose_num = true;
                    } else {
                        self.loading = true;
                        self.update_ui = true;
                        self.chose_num = false;
                        self.finished_num_fetch = false;
                        self.finished_name_fetch = false;
                        self.finished_sprite_fetch = false;
                        self.finished_shiny_sprite_fetch = false;
                        self.finished_ptype_fetch = false;
                        self.finished_stype_fetch = false;
                        self.num_mon = None;
                        self.stored_sprite = None;
                        self.stored_shiny_sprite = None;
                        self.stored_ptype = None;
                        self.stored_stype = None;
                        self.num_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.name_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.ptype_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.stype_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.sprite_web_req = Arc::new(Mutex::new(WebRequest::None));
                        self.shiny_sprite_web_req = Arc::new(Mutex::new(WebRequest::None));

                        let query = name_query::Variables {
                            pokemon: self.search.trim().to_owned(),
                        };

                        let name_request_body = NameQuery::build_query(query);
                        let name_request_body = serde_json::to_vec(&name_request_body);

                        let name_request = ehttp::Request {
                            headers: ehttp::headers(&[
                                ("Accept", "*/*"),
                                ("Content-Type", "application/json"),
                            ]),
                            ..ehttp::Request::post(
                                "https://graphqlpokemon.favware.tech/v7",
                                name_request_body.unwrap(),
                            )
                        };

                        let name_req_store = self.name_web_req.clone();
                        *name_req_store.lock().unwrap() = WebRequest::InProgress;
                        let ctx = ctx.clone();
                        ehttp::fetch(name_request, move |response| {
                            *name_req_store.lock().unwrap() = WebRequest::Done(response);
                            ctx.request_repaint(); // Wake up UI thread
                        });
                        self.chose_name = true;
                    }
                    self.search = "".to_owned();
                }
            });
            ui.horizontal(|ui| {
                ui.label("Species: ");
				ui.add_enabled_ui(self.enabled, |ui| {
					ui.label(&self.species);
					ui.add(egui::Image::new(
						self.ball.texture_id(ctx),
						egui::vec2(15.0, 15.0),
					)).on_hover_ui_at_pointer(|ui| {
						ui.add_sized(
							egui::vec2(200.0, 40.0),
							if self.num_mon.is_some() && self.name_mon.is_none() {
								egui::Label::new(format!("Base Catchrate: {}\nPoké Ball Catchrate: {}", self.num_mon.as_ref().unwrap().clone().catch_rate.unwrap().base, self.num_mon.as_ref().unwrap().clone().catch_rate.unwrap().percentage_with_ordinary_pokeball_at_full_health)).wrap(true)
							} else if self.num_mon.is_none() && self.name_mon.is_some() {
								egui::Label::new(format!("Base Catchrate: {}\nPoké Ball Catchrate: {}", self.name_mon.as_ref().unwrap().clone().catch_rate.unwrap().base, self.name_mon.as_ref().unwrap().clone().catch_rate.unwrap().percentage_with_ordinary_pokeball_at_full_health)).wrap(true)
							} else {
								egui::Label::new("").wrap(true)
							}
						);
					});
					ui.label(&self.gender_ratio);
				});
            });
            ui.horizontal(|ui| {
                ui.label("Types: ");
                ui.add(egui::Image::new(
                    self.ptype.texture_id(ctx),
                    egui::vec2(50.0, 11.0),
                ));
                ui.add(egui::Image::new(
                    self.stype.texture_id(ctx),
                    egui::vec2(50.0, 11.0),
                ));
            });
            ui.horizontal(|ui| {
                ui.label("Abilities: ");
                ui.add_enabled_ui(self.enabled, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                    ui.label(&self.abilities.primary_name)
                        .on_hover_ui_at_pointer(|ui| {
                            ui.add_sized(
                                egui::vec2(250.0, 50.0),
                                egui::Label::new(&self.abilities.primary_desc).wrap(true),
                            );
                        });
                    if self.abilities.secondary_name.is_some()
                        && self.abilities.secondary_desc.is_some()
                    {
                        ui.label(" / ");
                        ui.label(&self.abilities.secondary_name.as_ref().unwrap().clone())
                            .on_hover_ui_at_pointer(|ui| {
                                ui.add_sized(
                                    egui::vec2(250.0, 50.0),
                                    egui::Label::new(
                                        &self.abilities.secondary_desc.as_ref().unwrap().clone(),
                                    )
                                    .wrap(true),
                                );
                            });
                    }
                    if self.abilities.hidden_name.is_some() && self.abilities.hidden_desc.is_some()
                    {
                        ui.label(" | HA: ");
                        ui.label(&self.abilities.hidden_name.as_ref().unwrap().clone())
                            .on_hover_ui_at_pointer(|ui| {
                                ui.add_sized(
                                    egui::vec2(250.0, 50.0),
                                    egui::Label::new(
                                        &self.abilities.hidden_desc.as_ref().unwrap().clone(),
                                    )
                                    .wrap(true),
                                );
                            });
                    }
                });
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
                        if self.shiny == false {
                            self.sprite = RetainedImage::from_image_bytes(
                                "sprite.png",
                                self.stored_shiny_sprite.as_ref().unwrap(),
                            )
                            .unwrap();
                            self.shiny = true;
                        } else {
                            self.sprite = RetainedImage::from_image_bytes(
                                "sprite.png",
                                self.stored_sprite.as_ref().unwrap(),
                            )
                            .unwrap();
                            self.shiny = false;
                        }
                    }
					ui.vertical(|ui| {
						ui.horizontal(|ui| {
							ui.set_min_height(64.0);
							ui.add(egui::Label::new(&self.description).wrap(true));
						});
						ui.add_space(32.0);
						ui.horizontal(|ui| {
							ui.add(egui::Image::new(
								self.smogon.texture_id(ctx),
								egui::vec2(40.0, 32.0),
							)).on_hover_ui_at_pointer(|ui| {
								ui.add_sized(
									egui::vec2(150.0, 20.0),
									if self.num_mon.is_some() && self.name_mon.is_none() {
										egui::Label::new(format!("Smogon Tier: {}", self.num_mon.as_ref().unwrap().clone().smogon_tier)).wrap(true)
									} else if self.num_mon.is_none() && self.name_mon.is_some() {
										egui::Label::new(format!("Smogon Tier: {}", self.name_mon.as_ref().unwrap().clone().smogon_tier)).wrap(true)
									} else {
										egui::Label::new("").wrap(true)
									}
								);
							});
							/* if self.num_mon.is_some() && self.name_mon.is_none() || self.num_mon.is_none() && self.name_mon.is_some() {
								if self.num_mon.is_some() && self.name_mon.is_none() {
									if self.num_mon.as_ref().unwrap().is_egg_obtainable {
										ui.add(egui::Image::new(
											self.egg.texture_id(ctx),
											egui::vec2(32.0, 32.0),
										)).on_hover_ui_at_pointer(|ui| {
											ui.add_sized(
												egui::vec2(200.0, 40.0),
												egui::Label::new(format!("Egg Groups: {}\nMin Hatch Steps: {}\nMax Hatch Steps: {}", self.num_mon.as_ref().unwrap().clone().egg_groups.unwrap().join(" "), self.num_mon.as_ref().unwrap().clone().minimum_hatch_time.unwrap(), self.num_mon.as_ref().unwrap().clone().maximum_hatch_time.unwrap())).wrap(true)
											);
										});
									}
							} */
						});
					});
                });
            });
            ui.add(egui::Label::new(""));
            ui.horizontal(|ui| {
                ui.label("Powered by:");
                ui.hyperlink_to(
                    "graphqlpokemon.favware.tech",
                    "https://graphqlpokemon.favware.tech/v7",
                );
                if self.loading {
                    ui.spinner();
                }
            });
        });

        if self.finished_num_fetch == false && self.chose_num == true {
            let num_fetch: &WebRequest = &self.num_web_req.lock().unwrap();

            if let WebRequest::InProgress = num_fetch {
                self.loading = true;
            }
            if let WebRequest::Done(response) = num_fetch {
                let body = serde_json::from_slice::<
                    graphql_client::Response<num_query::ResponseData>,
                >(&response.as_ref().unwrap().bytes)
                .unwrap();
                let mon = body.data.unwrap().get_pokemon_by_dex_number;
                self.num_mon = Some(mon.clone());

                // fetch sprites once the mon is fetched
                let ctx = ctx.clone();
                let sprite_request = ehttp::Request {
                    headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "image/png")]),
                    ..ehttp::Request::get(&format!("https://dex.pkmn.dev/sprites/{}.png", mon.num))
                };
                let sprite_req_store = self.sprite_web_req.clone();
                *sprite_req_store.lock().unwrap() = WebRequest::InProgress;
                ehttp::fetch(sprite_request, move |response| {
                    *sprite_req_store.lock().unwrap() = WebRequest::Done(response);
                    ctx.request_repaint();
                });

                let shiny_sprite_request = ehttp::Request {
                    headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "image/png")]),
                    ..ehttp::Request::get(&format!(
                        "https://dex.pkmn.dev/sprites/shiny/{}.png",
                        mon.num
                    ))
                };
                let shiny_sprite_req_store = self.shiny_sprite_web_req.clone();
                *shiny_sprite_req_store.lock().unwrap() = WebRequest::InProgress;
                ehttp::fetch(shiny_sprite_request, move |response| {
                    *shiny_sprite_req_store.lock().unwrap() = WebRequest::Done(response);
                });

                let ptype_request = ehttp::Request {
                    headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "image/jpg")]),
                    ..ehttp::Request::get(
                        &format!(
                            "https://dex.pkmn.dev/types/{}.jpg",
                            case::lower_case(mon.types.get(0).unwrap().primary.as_str())
                        )
                        .to_string(),
                    )
                };
                let ptype_req_store = self.ptype_web_req.clone();
                *ptype_req_store.lock().unwrap() = WebRequest::InProgress;
                ehttp::fetch(ptype_request, move |response| {
                    *ptype_req_store.lock().unwrap() = WebRequest::Done(response);
                });

                if mon.types.get(1).is_some() {
                    let stype_request = ehttp::Request {
                        headers: ehttp::headers(&[
                            ("Accept", "*/*"),
                            ("Content-Type", "image/jpg"),
                        ]),
                        ..ehttp::Request::get(
                            &format!(
                                "https://dex.pkmn.dev/types/{}.jpg",
                                case::lower_case(mon.types.get(1).unwrap().primary.as_str())
                            )
                            .to_string(),
                        )
                    };
                    let stype_req_store = self.stype_web_req.clone();
                    *stype_req_store.lock().unwrap() = WebRequest::InProgress;
                    ehttp::fetch(stype_request, move |response| {
                        *stype_req_store.lock().unwrap() = WebRequest::Done(response);
                    });
                } else {
                    self.finished_stype_fetch = true;
                    self.stored_stype = Some(include_bytes!("../assets/icons/empty.png").to_vec());
                }

                self.finished_num_fetch = true;
            }
        }

        if self.finished_name_fetch == false && self.chose_name == true {
            let name_fetch: &WebRequest = &self.name_web_req.lock().unwrap();

            if let WebRequest::InProgress = name_fetch {
                self.loading = true;
            }
            if let WebRequest::Done(response) = name_fetch {
                let body = serde_json::from_slice::<
                    graphql_client::Response<name_query::ResponseData>,
                >(&response.as_ref().unwrap().bytes)
                .unwrap();
                let mon = body.data.unwrap().get_fuzzy_pokemon.get(0).unwrap().clone();
                self.name_mon = Some(mon.clone());

                // fetch sprites once the mon is fetched
                let ctx = ctx.clone();
                let sprite_request = ehttp::Request {
                    headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "image/png")]),
                    ..ehttp::Request::get(&format!("https://dex.pkmn.dev/sprites/{}.png", mon.num))
                };
                let sprite_req_store = self.sprite_web_req.clone();
                *sprite_req_store.lock().unwrap() = WebRequest::InProgress;
                ehttp::fetch(sprite_request, move |response| {
                    *sprite_req_store.lock().unwrap() = WebRequest::Done(response);
                    ctx.request_repaint();
                });

                let shiny_sprite_request = ehttp::Request {
                    headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "image/png")]),
                    ..ehttp::Request::get(&format!(
                        "https://dex.pkmn.dev/sprites/shiny/{}.png",
                        mon.num
                    ))
                };
                let shiny_sprite_req_store = self.shiny_sprite_web_req.clone();
                *shiny_sprite_req_store.lock().unwrap() = WebRequest::InProgress;
                ehttp::fetch(shiny_sprite_request, move |response| {
                    *shiny_sprite_req_store.lock().unwrap() = WebRequest::Done(response);
                });

                let ptype_request = ehttp::Request {
                    headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "image/jpg")]),
                    ..ehttp::Request::get(
                        &format!(
                            "https://dex.pkmn.dev/types/{}.jpg",
                            case::lower_case(mon.types.get(0).unwrap().primary.as_str())
                        )
                        .to_string(),
                    )
                };
                let ptype_req_store = self.ptype_web_req.clone();
                *ptype_req_store.lock().unwrap() = WebRequest::InProgress;
                ehttp::fetch(ptype_request, move |response| {
                    *ptype_req_store.lock().unwrap() = WebRequest::Done(response);
                });

                if mon.types.get(1).is_some() {
                    let stype_request = ehttp::Request {
                        headers: ehttp::headers(&[
                            ("Accept", "*/*"),
                            ("Content-Type", "image/jpg"),
                        ]),
                        ..ehttp::Request::get(
                            &format!(
                                "https://dex.pkmn.dev/types/{}.jpg",
                                case::lower_case(mon.types.get(1).unwrap().primary.as_str())
                            )
                            .to_string(),
                        )
                    };
                    let stype_req_store = self.stype_web_req.clone();
                    *stype_req_store.lock().unwrap() = WebRequest::InProgress;
                    ehttp::fetch(stype_request, move |response| {
                        *stype_req_store.lock().unwrap() = WebRequest::Done(response);
                    });
                } else {
                    self.finished_stype_fetch = true;
                    self.stored_stype = Some(include_bytes!("../assets/icons/empty.png").to_vec());
                }

                self.finished_name_fetch = true;
            }
        }

        // check if the sprite request is done
        if self.finished_sprite_fetch == false && self.finished_num_fetch == true
            || self.finished_sprite_fetch == false && self.finished_name_fetch == true
        {
            let sprite_fetch: &WebRequest = &self.sprite_web_req.lock().unwrap();

            if let WebRequest::InProgress = sprite_fetch {
                self.loading = true;
            }

            if let WebRequest::Done(response) = sprite_fetch {
                let bytes = response.as_ref().unwrap().bytes.to_vec();
                self.stored_sprite = Some(bytes);
                self.finished_sprite_fetch = true;
            }
        }
        // check if the shiny sprite request is done
        if self.finished_shiny_sprite_fetch == false && self.finished_num_fetch == true
            || self.finished_shiny_sprite_fetch == false && self.finished_name_fetch == true
        {
            let shiny_sprite_fetch: &WebRequest = &self.shiny_sprite_web_req.lock().unwrap();

            if let WebRequest::InProgress = shiny_sprite_fetch {
                self.loading = true;
            }

            if let WebRequest::Done(response) = shiny_sprite_fetch {
                let bytes = response.as_ref().unwrap().bytes.to_vec();
                self.stored_shiny_sprite = Some(bytes);
                self.finished_shiny_sprite_fetch = true;
            }
        }
        // check if the ptype icon request is done
        if self.finished_ptype_fetch == false && self.finished_num_fetch == true
            || self.finished_ptype_fetch == false && self.finished_name_fetch == true
        {
            let ptype_fetch: &WebRequest = &self.ptype_web_req.lock().unwrap();

            if let WebRequest::InProgress = ptype_fetch {
                self.loading = true;
            }

            if let WebRequest::Done(response) = ptype_fetch {
                let bytes = response.as_ref().unwrap().bytes.to_vec();
                self.stored_ptype = Some(bytes);
                self.finished_ptype_fetch = true;
            }
        }
        // check if the stype icon request is done
        if self.finished_stype_fetch == false && self.finished_num_fetch == true
            || self.finished_stype_fetch == false && self.finished_name_fetch == true
        {
            let stype_fetch: &WebRequest = &self.stype_web_req.lock().unwrap();

            if let WebRequest::InProgress = stype_fetch {
                self.loading = true;
            }

            if let WebRequest::Done(response) = stype_fetch {
                let bytes = response.as_ref().unwrap().bytes.to_vec();
                self.stored_stype = Some(bytes);
                self.finished_stype_fetch = true;
            }
        }

        // update ui when num_mon and sprites are fetched
        if self.num_mon.is_some()
            && self.stored_sprite.is_some()
            && self.stored_shiny_sprite.is_some()
            && self.stored_ptype.is_some()
            && self.stored_stype.is_some()
            && self.update_ui == true
        {
            let mon = self.num_mon.as_ref().unwrap();

            self.species = format!(
                "#{} {}",
                mon.num,
                case::capitalize(&mon.species, true),
            )
            .to_owned();
			self.gender_ratio = format!(
                "♂: {} ♀: {}",
                mon.gender.male,
                mon.gender.female
            )
            .to_owned();
			self.ball = RetainedImage::from_image_bytes(
                "ball.png",
                include_bytes!("../assets/icons/ball.png"),
            )
            .unwrap();
			self.smogon = RetainedImage::from_image_bytes(
                "smogon.png",
                include_bytes!("../assets/icons/smogon.png"),
            )
            .unwrap();
			self.egg = RetainedImage::from_image_bytes(
                "egg.png",
                include_bytes!("../assets/icons/egg.png"),
            )
            .unwrap();
            self.description = mon.flavor_texts.get(0).unwrap().flavor.clone();
            self.sprite =
                RetainedImage::from_image_bytes("sprite.png", self.stored_sprite.as_ref().unwrap())
                    .unwrap();
            self.ptype = RetainedImage::from_image_bytes(
                "ptype.jpg",
                self.stored_ptype.as_ref().unwrap().as_slice(),
            )
            .unwrap();
            self.stype = RetainedImage::from_image_bytes(
                "stype.jpg",
                self.stored_stype.as_ref().unwrap().as_slice(),
            )
            .unwrap();
            self.abilities = Ablities {
                primary_name: mon.abilities.first.name.clone(),
                primary_desc: mon.abilities.first.short_desc.clone(),
                secondary_name: match mon.abilities.second.is_some() {
                    true => Some(mon.abilities.second.as_ref().unwrap().name.clone()),
                    false => None,
                },
                secondary_desc: match mon.abilities.second.is_some() {
                    true => Some(mon.abilities.second.as_ref().unwrap().short_desc.clone()),
                    false => None,
                },
                hidden_name: match mon.abilities.hidden.is_some() {
                    true => Some(mon.abilities.hidden.as_ref().unwrap().name.clone()),
                    false => None,
                },
                hidden_desc: match mon.abilities.hidden.is_some() {
                    true => Some(mon.abilities.hidden.as_ref().unwrap().short_desc.clone()),
                    false => None,
                },
            };
            self.dimensions =
                format!("Height: {} M | Weight: {} KG", mon.height, mon.weight).to_owned();
            self.enabled = true;
            self.shiny = false;
            self.loading = false;
            self.update_ui = false;
        }

        // update ui when name_mon and sprites are fetched
        if self.name_mon.is_some()
            && self.stored_sprite.is_some()
            && self.stored_shiny_sprite.is_some()
            && self.stored_ptype.is_some()
            && self.stored_stype.is_some()
            && self.update_ui == true
        {
            let mon = self.name_mon.as_ref().unwrap();

            self.species = format!(
                "#{} {}",
                mon.num,
                case::capitalize(&mon.species, true),
            )
            .to_owned();
			self.gender_ratio = format!(
                "♂: {} ♀: {}",
                mon.gender.male,
                mon.gender.female
            )
            .to_owned();
			self.ball = RetainedImage::from_image_bytes(
                "ball.png",
                include_bytes!("../assets/icons/ball.png"),
            )
            .unwrap();
			self.smogon = RetainedImage::from_image_bytes(
                "smogon.png",
                include_bytes!("../assets/icons/smogon.png"),
            )
            .unwrap();
            self.description = mon.flavor_texts.get(0).unwrap().flavor.clone();
            self.sprite =
                RetainedImage::from_image_bytes("sprite.png", self.stored_sprite.as_ref().unwrap())
                    .unwrap();
            self.ptype = RetainedImage::from_image_bytes(
                "ptype.jpg",
                self.stored_ptype.as_ref().unwrap().as_slice(),
            )
            .unwrap();
            self.stype = RetainedImage::from_image_bytes(
                "stype.jpg",
                self.stored_stype.as_ref().unwrap().as_slice(),
            )
            .unwrap();
            self.abilities = Ablities {
                primary_name: mon.abilities.first.name.clone(),
                primary_desc: mon.abilities.first.short_desc.clone(),
                secondary_name: match mon.abilities.second.is_some() {
                    true => Some(mon.abilities.second.as_ref().unwrap().name.clone()),
                    false => None,
                },
                secondary_desc: match mon.abilities.second.is_some() {
                    true => Some(mon.abilities.second.as_ref().unwrap().short_desc.clone()),
                    false => None,
                },
                hidden_name: match mon.abilities.hidden.is_some() {
                    true => Some(mon.abilities.hidden.as_ref().unwrap().name.clone()),
                    false => None,
                },
                hidden_desc: match mon.abilities.hidden.is_some() {
                    true => Some(mon.abilities.hidden.as_ref().unwrap().short_desc.clone()),
                    false => None,
                },
            };
            self.dimensions =
                format!("Height: {} M | Weight: {} KG", mon.height, mon.weight).to_owned();
            self.enabled = true;
            self.shiny = false;
            self.loading = false;
            self.update_ui = false;
        }
    }
}
