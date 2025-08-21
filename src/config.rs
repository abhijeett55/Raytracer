use jpeg_decoder:: Decoder;
use palette::Srgb;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use std::fs::File;
use std::io::BufReader;
#[cfg(test)]
use std::fs;

#[serde_with::serde_as]
#[derive(Debug, Serialize , Deserailize)]

pub struct Sky {
    #[serde_as(as = "TextureOptionPixelsAsPath")]
    pub textute: Option<(Vec<u8>, usize, usize, String)>,
}

impl Sky {
    pub fn new_default_sky() -> Sky {
        Sky  { textute: None }
    }
}

fn load_texture_image(path: &str) -> (Vec<u8>, usize, usize, String) {
    let file = File::open(path).expect(path);
    let mut decoder = Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let mutdata = decoder.info().unwrap();
    (
        pixels,
        metadata.width as usize,
        metadata.height as usize,
        path.to_string(),
        )
}
