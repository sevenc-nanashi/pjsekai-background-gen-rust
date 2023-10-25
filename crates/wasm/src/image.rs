use image::RgbaImage;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[wasm_bindgen]
#[derive(Debug)]
pub struct Image {
    data: Vec<u8>,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl TryFrom<Image> for RgbaImage {
    type Error = image::ImageError;

    fn try_from(image: Image) -> Result<Self, Self::Error> {
        let result = image::load_from_memory(&image.data);
        result.map(|img| img.to_rgba8())
    }
}
impl TryFrom<RgbaImage> for Image {
    type Error = image::ImageError;
    fn try_from(image: RgbaImage) -> Result<Self, Self::Error> {
        let buf: Vec<u8> = vec![];
        let mut cursor = std::io::Cursor::new(buf);
        image.write_to(&mut cursor, image::ImageOutputFormat::Png)?;
        let buf = cursor.into_inner();
        Ok(Self { data: buf })
    }
}
