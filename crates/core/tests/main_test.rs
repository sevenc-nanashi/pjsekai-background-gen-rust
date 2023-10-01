extern crate pjsekai_background_gen_core;

#[test]
fn test() {
    let image = image::load_from_memory(include_bytes!("../test.png"))
        .unwrap()
        .into_rgba8();
    let result = pjsekai_background_gen_core::render(&image);
    result.save("../../dist/latest.png").unwrap();
}

#[test]
fn test_v3() {
    let image = image::load_from_memory(include_bytes!("../test.png"))
        .unwrap()
        .into_rgba8();
    let result = pjsekai_background_gen_core::render_v3(&image);
    result.save("../../dist/v3.png").unwrap();
}

#[test]
fn test_v1() {
    let image = image::load_from_memory(include_bytes!("../test.png"))
        .unwrap()
        .into_rgba8();
    let result = pjsekai_background_gen_core::render_v1(&image);
    result.save("../../dist/v1.png").unwrap();
}
