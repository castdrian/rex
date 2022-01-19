use viuer::{Config, print};
use std::error::Error;

fn fetch_image(url: &str) -> Result<image::DynamicImage, Box<dyn Error>>{
	let img_bytes = reqwest::blocking::get(url)?.bytes()?;
	let image = image::load_from_memory(&img_bytes)?;

	Ok(image)
}

pub fn show_sprite(sprite: &str, width: Option<u32>, height: Option<u32>, x: u16, y: i16) {
	let image = fetch_image(sprite).expect("Failed to fetch image");

	let conf = Config {
		absolute_offset: false,
		restore_cursor: true,
		transparent: true,
		truecolor: true,
		width: width,
		height: height,
		x: x,
		y: y,
		..Default::default()
	};

	print(&image, &conf).expect("Image printing failed.");
}