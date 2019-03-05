use crate::colors::*;
use image::{
    imageops::{self, colorops::ColorMap},
    DynamicImage, ImageBuffer, Pixel, Rgb,
};
pub type Image = ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub fn convert(image: DynamicImage, dither: bool, width: u32, height: u32) -> Image {
    let rgba = image.to_rgba();

    // canvas is x814y611, but a pixel is 3x3
    let (thumbnail_x, thumbnail_y) =
        resize_dimensions(rgba.width(), rgba.height(), width / 3, height / 3, false);
    let thumbnail = imageops::thumbnail(&rgba, thumbnail_x, thumbnail_y);
    let mut rgb = ImageBuffer::new(thumbnail.width(), thumbnail.height());

    for (rgb_pixel, thumbnail_pixel) in rgb.pixels_mut().zip(thumbnail.pixels()) {
        let thumbnail_pixel_rgba = thumbnail_pixel.to_rgba().data;
        *rgb_pixel = if thumbnail_pixel_rgba[3] != 255 {
            Rgb {
                data: [255, 255, 255],
            }
        } else {
            Rgb {
                data: [
                    thumbnail_pixel_rgba[0],
                    thumbnail_pixel_rgba[1],
                    thumbnail_pixel_rgba[2],
                ],
            }
        }
    }

    let color_map = SkribblColorMap;
    if dither {
        imageops::dither(&mut rgb, &color_map);
    } else {
        for pixel in rgb.pixels_mut() {
            let mut new = pixel.to_rgb();
            color_map.map_color(&mut new);
            *pixel = new;
        }
    }

    rgb
}

// Copied from image source code
fn resize_dimensions(width: u32, height: u32, nwidth: u32, nheight: u32, fill: bool) -> (u32, u32) {
    let ratio = u64::from(width) * u64::from(nheight);
    let nratio = u64::from(nwidth) * u64::from(height);

    let use_width = if fill {
        nratio > ratio
    } else {
        nratio <= ratio
    };
    let intermediate = if use_width {
        u64::from(height) * u64::from(nwidth) / u64::from(width)
    } else {
        u64::from(width) * u64::from(nheight) / u64::from(height)
    };
    if use_width {
        if intermediate <= u64::from(::std::u32::MAX) {
            (nwidth, intermediate as u32)
        } else {
            (
                (u64::from(nwidth) * u64::from(::std::u32::MAX) / intermediate) as u32,
                ::std::u32::MAX,
            )
        }
    } else if intermediate <= u64::from(::std::u32::MAX) {
        (intermediate as u32, nheight)
    } else {
        (
            ::std::u32::MAX,
            (u64::from(nheight) * u64::from(::std::u32::MAX) / intermediate) as u32,
        )
    }
}

pub struct SkribblColorMap;

impl ColorMap for SkribblColorMap {
    type Color = Rgb<u8>;

    // we are not implementing this, because it's unused by the dithering function
    fn index_of(&self, _: &Rgb<u8>) -> usize {
        0
    }

    fn map_color(&self, color: &mut Rgb<u8>) {
        let mut diffs = vec![
            difference(color.data, WHITE),
            difference(color.data, LIGHT_GREY),
            difference(color.data, LIGHT_RED),
            difference(color.data, LIGHT_ORANGE),
            difference(color.data, LIGHT_YELLOW),
            difference(color.data, LIGHT_GREEN),
            difference(color.data, LIGHT_CYAN),
            difference(color.data, LIGHT_BLUE),
            difference(color.data, LIGHT_MAGENTA),
            difference(color.data, LIGHT_PINK),
            difference(color.data, LIGHT_BROWN),
            difference(color.data, BLACK),
            difference(color.data, DARK_GREY),
            difference(color.data, DARK_RED),
            difference(color.data, DARK_ORANGE),
            difference(color.data, DARK_YELLOW),
            difference(color.data, DARK_GREEN),
            difference(color.data, DARK_CYAN),
            difference(color.data, DARK_BLUE),
            difference(color.data, DARK_MAGENTA),
            difference(color.data, DARK_PINK),
            difference(color.data, DARK_BROWN),
        ];

        diffs.sort_by(|a, b| a.0.cmp(&b.0));

        let best_color = &diffs[0].1;
        color.data[0] = best_color.r;
        color.data[1] = best_color.g;
        color.data[2] = best_color.b;
    }
}

fn difference(image_color: [u8; 3], skribbl_color: Color) -> (i32, Color) {
    let image_r = i32::from(image_color[0]);
    let image_g = i32::from(image_color[1]);
    let image_b = i32::from(image_color[2]);

    let skribbl_r = i32::from(skribbl_color.r);
    let skribbl_g = i32::from(skribbl_color.g);
    let skribbl_b = i32::from(skribbl_color.b);

    let diff =
        (image_r - skribbl_r).abs() + (image_g - skribbl_g).abs() + (image_b - skribbl_b).abs();

    (diff as i32, skribbl_color)
}
