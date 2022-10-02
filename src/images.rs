use egui_extras::RetainedImage;
use poll_promise::Promise;
use std::error::Error;

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
