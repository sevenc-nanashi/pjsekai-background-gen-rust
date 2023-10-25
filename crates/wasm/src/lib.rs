mod image;
use ::image::RgbaImage;
use duplicate::duplicate_item;
use pjsekai_background_gen_core as core;
use wasm_bindgen::prelude::*;

use crate::image::Image;

#[duplicate_item(
    js_method_name method;
    [renderV1]     [render_v1];
    [renderV3]     [render_v3];
)]
#[wasm_bindgen(js_name = js_method_name)]
pub fn method(base: Image) -> Result<Image, JsValue> {
    let image = RgbaImage::try_from(base)
        .map_err(|err| JsValue::from(format!("failed to convert image: {}", err)))?;

    let rendered = core::method(&image);

    rendered
        .try_into()
        .map_err(|err| JsValue::from(format!("failed to convert image: {}", err)))
}

#[wasm_bindgen]
pub fn render(base: Image) -> Result<Image, JsValue> {
    render_v3(base)
}
