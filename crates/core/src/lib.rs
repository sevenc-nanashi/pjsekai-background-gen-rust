mod assets;
use image::Rgba;
use imageproc::{geometric_transformations as transform, map::map_pixels_mut};

use crate::assets::{V1_ASSETS, V3_ASSETS};

type Position = (u32, u32);

fn morph(
    image: &image::RgbaImage,
    target: [Position; 4],
    target_size: (u32, u32),
) -> image::RgbaImage {
    let min_x = target.iter().map(|p| p.0).min().unwrap();
    let min_y = target.iter().map(|p| p.1).min().unwrap();
    let max_x = target.iter().map(|p| p.0).max().unwrap();
    let max_y = target.iter().map(|p| p.1).max().unwrap();
    let min_image = image::imageops::resize(
        image,
        max_x - min_x,
        max_y - min_y,
        image::imageops::FilterType::Nearest,
    );
    let projection = transform::Projection::from_control_points(
        [
            (0.0, 0.0),
            (min_image.width() as f32, 0.0),
            (0.0, min_image.height() as f32),
            (min_image.width() as f32, min_image.height() as f32),
        ],
        [
            ((target[0].0 - min_x) as f32, (target[0].1 - min_y) as f32),
            ((target[1].0 - min_x) as f32, (target[1].1 - min_y) as f32),
            ((target[2].0 - min_x) as f32, (target[2].1 - min_y) as f32),
            ((target[3].0 - min_x) as f32, (target[3].1 - min_y) as f32),
        ],
    )
    .unwrap();
    let projected = transform::warp(
        &min_image,
        &projection,
        transform::Interpolation::Nearest,
        image::Rgba([0, 0, 0, 0]),
    );
    let mut target = image::RgbaImage::new(target_size.0, target_size.1);
    image::imageops::overlay(&mut target, &projected, min_x as _, min_y as _);
    target
}

fn mask(image: &image::RgbaImage, mask: &image::RgbaImage) -> image::RgbaImage {
    let mut masked = image.clone();
    map_pixels_mut(&mut masked, |x, y, pixel| {
        Rgba([
            pixel[0],
            pixel[1],
            pixel[2],
            *[mask.get_pixel(x, y).0[3], pixel[3]].iter().min().unwrap(),
        ])
    });
    masked
}

pub fn render(target: &image::RgbaImage) -> image::RgbaImage {
    render_v3(target)
}

pub fn render_v3(target: &image::RgbaImage) -> image::RgbaImage {
    let mut base = V3_ASSETS.base.clone();
    let mut side_jackets = image::RgbaImage::new(base.width(), base.height());
    let left_normal = morph(
        target,
        [(566, 161), (1183, 134), (633, 731), (1226, 682)],
        (base.width(), base.height()),
    );

    let right_normal = morph(
        target,
        [(966, 104), (1413, 72), (954, 525), (1390, 524)],
        (base.width(), base.height()),
    );

    let left_mirror = morph(
        target,
        [(633, 1071), (1256, 1045), (598, 572), (1197, 569)],
        (base.width(), base.height()),
    );

    let right_mirror = morph(
        target,
        [(954, 1122), (1393, 1167), (942, 702), (1366, 717)],
        (base.width(), base.height()),
    );

    image::imageops::overlay(&mut side_jackets, &left_normal, 0, 0);
    image::imageops::overlay(&mut side_jackets, &right_normal, 0, 0);
    image::imageops::overlay(&mut side_jackets, &left_mirror, 0, 0);
    image::imageops::overlay(&mut side_jackets, &right_mirror, 0, 0);

    image::imageops::overlay(&mut side_jackets, &V3_ASSETS.side_cover, 0, 0);

    let center_normal = morph(
        target,
        [(824, 227), (1224, 227), (833, 608), (1216, 608)],
        (base.width(), base.height()),
    );

    let center_mirror = morph(
        target,
        [(830, 1017), (1214, 1017), (833, 676), (1216, 676)],
        (base.width(), base.height()),
    );

    let mut center = image::RgbaImage::new(base.width(), base.height());
    image::imageops::overlay(&mut center, &center_normal, 0, 0);
    image::imageops::overlay(&mut center, &center_mirror, 0, 0);

    image::imageops::overlay(&mut center, &V3_ASSETS.center_cover, 0, 0);

    let side_jackets = mask(&side_jackets, &V3_ASSETS.side_mask);
    let center = mask(&center, &V3_ASSETS.center_mask);

    image::imageops::overlay(&mut base, &side_jackets, 0, 0);
    image::imageops::overlay(&mut base, &V3_ASSETS.side_cover, 0, 0);
    image::imageops::overlay(&mut base, &V3_ASSETS.windows, 0, 0);
    image::imageops::overlay(&mut base, &center, 0, 0);
    image::imageops::overlay(&mut base, &V3_ASSETS.bottom, 0, 0);

    base
}

pub fn render_v1(target: &image::RgbaImage) -> image::RgbaImage {
    let mut base = V1_ASSETS.base.clone();
    let mut side_jackets = image::RgbaImage::new(base.width(), base.height());
    let left_normal = morph(
        target,
        [(449, 114), (1136, 99), (465, 804), (1152, 789)],
        (base.width(), base.height()),
    );

    let right_normal = morph(
        target,
        [(1018, 92), (1635, 51), (1026, 756), (1630, 740)],
        (base.width(), base.height()),
    );

    image::imageops::overlay(&mut side_jackets, &left_normal, 0, 0);
    image::imageops::overlay(&mut side_jackets, &right_normal, 0, 0);

    let center_normal = morph(
        target,
        [(798, 193), (1252, 193), (801, 635), (1246, 635)],
        (base.width(), base.height()),
    );

    let center_mirror = morph(
        target,
        [(798, 1152), (1252, 1152), (795, 713), (1252, 713)],
        (base.width(), base.height()),
    );

    let mut center = image::RgbaImage::new(base.width(), base.height());

    let center_normal = mask(&center_normal, &V1_ASSETS.center_mask);
    let center_mirror = mask(&center_mirror, &V1_ASSETS.mirror_mask);
    image::imageops::overlay(&mut center, &center_normal, 0, 0);
    image::imageops::overlay(&mut center, &center_mirror, 0, 0);

    let side_jackets = mask(&side_jackets, &V1_ASSETS.side_mask);

    image::imageops::overlay(&mut base, &side_jackets, 0, 0);
    image::imageops::overlay(&mut base, &center, 0, 0);
    image::imageops::overlay(&mut base, &V1_ASSETS.frames, 0, 0);

    base
}
