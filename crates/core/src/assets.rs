use once_cell::sync::Lazy;

pub static V3_ASSETS: Lazy<V3Assets> = Lazy::new(|| V3Assets {
    base: image::load_from_memory(include_bytes!("../assets/v3/base.png"))
        .unwrap()
        .into_rgba8(),
    bottom: image::load_from_memory(include_bytes!("../assets/v3/bottom.png"))
        .unwrap()
        .into_rgba8(),
    center_cover: image::load_from_memory(include_bytes!("../assets/v3/center_cover.png"))
        .unwrap()
        .into_rgba8(),
    center_mask: image::load_from_memory(include_bytes!("../assets/v3/center_mask.png"))
        .unwrap()
        .into_rgba8(),
    side_cover: image::load_from_memory(include_bytes!("../assets/v3/side_cover.png"))
        .unwrap()
        .into_rgba8(),
    side_mask: image::load_from_memory(include_bytes!("../assets/v3/side_mask.png"))
        .unwrap()
        .into_rgba8(),
    windows: image::load_from_memory(include_bytes!("../assets/v3/windows.png"))
        .unwrap()
        .into_rgba8(),
});

pub struct V3Assets {
    pub base: image::RgbaImage,
    pub bottom: image::RgbaImage,
    pub center_cover: image::RgbaImage,
    pub center_mask: image::RgbaImage,
    pub side_cover: image::RgbaImage,
    pub side_mask: image::RgbaImage,
    pub windows: image::RgbaImage,
}

pub static V1_ASSETS: Lazy<V1Assets> = Lazy::new(|| V1Assets {
    base: image::load_from_memory(include_bytes!("../assets/v1/base.png"))
        .unwrap()
        .into_rgba8(),
    side_mask: image::load_from_memory(include_bytes!("../assets/v1/side_mask.png"))
        .unwrap()
        .into_rgba8(),
    center_mask: image::load_from_memory(include_bytes!("../assets/v1/center_mask.png"))
        .unwrap()
        .into_rgba8(),
    mirror_mask: image::load_from_memory(include_bytes!("../assets/v1/mirror_mask.png"))
        .unwrap()
        .into_rgba8(),
    frames: image::load_from_memory(include_bytes!("../assets/v1/frames.png"))
        .unwrap()
        .into_rgba8(),
});

pub struct V1Assets {
    pub base: image::RgbaImage,
    pub side_mask: image::RgbaImage,
    pub mirror_mask: image::RgbaImage,
    pub center_mask: image::RgbaImage,
    pub frames: image::RgbaImage,
}
