use viuer::{Config, print};
use std::error::Error;

fn fetch_image(url: &str) -> Result<image::DynamicImage, Box<dyn Error>>{
	let img_bytes = reqwest::blocking::get(url)?.bytes()?;
	let image = image::load_from_memory(&img_bytes)?;

	Ok(image)
}

pub fn show_sprite(sprite: &str) {
	let image = fetch_image(sprite).expect("Failed to fetch image");

	let conf = Config {
		absolute_offset: false,
		restore_cursor: true,
		transparent: true,
		truecolor: true,
		width: Some(28),
		height: Some(14),
		x: 40,
		y: 0,
		..Default::default()
	};

	print(&image, &conf).expect("Image printing failed.");
}