use poll_promise::Promise;
use std::error::Error;
use viuer::{print, Config};

fn fetch_image(url: &str) -> Result<image::DynamicImage, Box<dyn Error>> {
    let mut promise: Option<Promise<image::DynamicImage>> = None;

    let result = promise.get_or_insert_with(|| {
        let (sender, promise) = Promise::new();
        let request = ehttp::Request::get(url);
        ehttp::fetch(request, move |response| {
            let image = image::load_from_memory(&response.unwrap().bytes).unwrap();
            sender.send(image);
        });
        promise
    });

    Ok(result.block_until_ready().clone())
}

pub fn fetch_image_bytes(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut promise: Option<Promise<Vec<u8>>> = None;

    let result = promise.get_or_insert_with(|| {
        let (sender, promise) = Promise::new();
        let request = ehttp::Request::get(url);
        ehttp::fetch(request, move |response| {
            let bytes = response.unwrap().bytes.to_vec();
            sender.send(bytes);
        });
        promise
    });

    Ok(result.block_until_ready().clone())
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
