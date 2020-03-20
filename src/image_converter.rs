use crate::colors::*;
use image::{
    imageops::{self, colorops::ColorMap},
    DynamicImage, ImageBuffer, ImageFormat, Pixel, Rgb,
};
use std::process::Command;

pub type Image = ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub fn convert(
    image: DynamicImage,
    dither: bool,
    grayscale: bool,
    scale: f64,
    width: u32,
    height: u32,
) -> Image {
    let rgba = image.to_rgba();

    // canvas is x814y611, but a pixel is 3x3
    let (thumbnail_x, thumbnail_y) =
        resize_dimensions(rgba.width(), rgba.height(), width / 3, height / 3, false);
    let thumbnail = imageops::thumbnail(
        &rgba,
        (f64::from(thumbnail_x) * scale) as u32,
        (f64::from(thumbnail_y) * scale) as u32,
    );

    let mut rgb = ImageBuffer::new(thumbnail.width(), thumbnail.height());

    for (rgb_pixel, thumbnail_pixel) in rgb.pixels_mut().zip(thumbnail.pixels()) {
        let thumbnail_pixel_rgba = thumbnail_pixel.to_rgba().0;
        *rgb_pixel = if thumbnail_pixel_rgba[3] != 255 {
            Rgb([255, 255, 255])
        } else {
            Rgb([
                thumbnail_pixel_rgba[0],
                thumbnail_pixel_rgba[1],
                thumbnail_pixel_rgba[2],
            ])
        }
    }

    if grayscale {
        let grayscale = imageops::grayscale(&rgb);

        for (rgb_pixel, grayscale_pixel) in rgb.pixels_mut().zip(grayscale.pixels()) {
            *rgb_pixel = grayscale_pixel.to_rgb();
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

pub fn image_from_clipboard() -> Result<DynamicImage, String> {
    let xclip = Command::new("sh")
        .arg("-c")
        .arg("xclip -o -target image/png -selection clipboard")
        .output()
        .expect("failed to execute process");

    image::load_from_memory_with_format(&xclip.stdout, ImageFormat::Png).map_err(|_| {
        format!(
            "Clipboard error: {}",
            String::from_utf8_lossy(&xclip.stderr)
        )
    })
}

// Copied from image source code
pub fn resize_dimensions(
    width: u32,
    height: u32,
    nwidth: u32,
    nheight: u32,
    fill: bool,
) -> (u32, u32) {
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
            difference(color.0, WHITE),
            difference(color.0, LIGHT_GREY),
            difference(color.0, LIGHT_RED),
            difference(color.0, LIGHT_ORANGE),
            difference(color.0, LIGHT_YELLOW),
            difference(color.0, LIGHT_GREEN),
            difference(color.0, LIGHT_CYAN),
            difference(color.0, LIGHT_BLUE),
            difference(color.0, LIGHT_MAGENTA),
            difference(color.0, LIGHT_PINK),
            difference(color.0, LIGHT_BROWN),
            difference(color.0, BLACK),
            difference(color.0, DARK_GREY),
            difference(color.0, DARK_RED),
            difference(color.0, DARK_ORANGE),
            difference(color.0, DARK_YELLOW),
            difference(color.0, DARK_GREEN),
            difference(color.0, DARK_CYAN),
            difference(color.0, DARK_BLUE),
            difference(color.0, DARK_MAGENTA),
            difference(color.0, DARK_PINK),
            difference(color.0, DARK_BROWN),
        ];

        diffs.sort_by(|a, b| a.0.cmp(&b.0));

        let best_color = &diffs[0].1;
        color.0[0] = best_color.r;
        color.0[1] = best_color.g;
        color.0[2] = best_color.b;
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
