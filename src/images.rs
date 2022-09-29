use viuer::{Config, print};
use std::error::Error;

#[tokio::main]
async fn fetch_image(url: &str) -> Result<image::DynamicImage, Box<dyn Error>>{
	let img_bytes = reqwest::get(url).await?.bytes().await?;
	let image = image::load_from_memory(&img_bytes)?;
	Ok(image)
}

#[tokio::main]
pub async fn fetch_image_bytes(url: &str) -> Result<Vec<u8>, Box<dyn Error>>{
	let img_bytes = reqwest::get(url).await?.bytes().await?;
	Ok(img_bytes.to_vec())
}

pub fn show_sprite(sprite: &str, width: Option<u32>, height: Option<u32>, x: u16, y: i16) {
	let image = fetch_image(sprite).expect("Failed to fetch image");

	let conf = Config {
		absolute_offset: false,
		restore_cursor: true,
		transparent: true,
		truecolor: true,
		width,
		height,
		x,
		y,
		..Default::default()
	};

	print(&image, &conf).expect("Image printing failed.");
}