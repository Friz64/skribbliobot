use crate::{
    cli::Box,
    colors::{Color, ColorCoord, *},
    desktop::{ClickType, Desktop},
    image_converter::Image,
};
use image::Pixel;
use std::{collections::HashMap, thread, time::Duration};

fn calculate_color_positions(color_box: Box) -> HashMap<Color, ColorCoord> {
    let mut map = HashMap::new();

    let xmiddle = color_box.x + (color_box.width / 2);
    let ymiddle = color_box.y + (color_box.height / 2);

    for x in 0..11 {
        for y in 0..2 {
            let xpos = xmiddle + (color_box.width * x);
            let ypos = ymiddle + (color_box.height * y);

            let color = match (x, y) {
                (0, 0) => WHITE,
                (1, 0) => LIGHT_GREY,
                (2, 0) => LIGHT_RED,
                (3, 0) => LIGHT_ORANGE,
                (4, 0) => LIGHT_YELLOW,
                (5, 0) => LIGHT_GREEN,
                (6, 0) => LIGHT_CYAN,
                (7, 0) => LIGHT_BLUE,
                (8, 0) => LIGHT_MAGENTA,
                (9, 0) => LIGHT_PINK,
                (10, 0) => LIGHT_BROWN,
                (0, 1) => BLACK,
                (1, 1) => DARK_GREY,
                (2, 1) => DARK_RED,
                (3, 1) => DARK_ORANGE,
                (4, 1) => DARK_YELLOW,
                (5, 1) => DARK_GREEN,
                (6, 1) => DARK_CYAN,
                (7, 1) => DARK_BLUE,
                (8, 1) => DARK_MAGENTA,
                (9, 1) => DARK_PINK,
                (10, 1) => DARK_BROWN,
                (_, _) => unreachable!(),
            };

            map.insert(color, ColorCoord { x: xpos, y: ypos });
        }
    }

    map
}

pub struct Drawer {
    xoff: u32,
    yoff: u32,
    checkerboard: bool,
    batch_colors: bool,
    delay: u64,
    timeout: u64,
    color_map: HashMap<Color, ColorCoord>,
    last_color: ColorCoord,
}

impl Drawer {
    pub fn new(
        xoff: u32,
        yoff: u32,
        checkerboard: bool,
        batch_colors: bool,
        delay: u64,
        timeout: u64,
        color_box: Box,
    ) -> Drawer {
        Drawer {
            xoff,
            yoff,
            checkerboard,
            batch_colors,
            delay,
            timeout,
            color_map: calculate_color_positions(color_box),
            last_color: ColorCoord { x: 0, y: 0 },
        }
    }

    fn draw_color(&mut self, desktop: &Desktop, color: Color, x: u32, y: u32) {
        if color == WHITE {
            return;
        }

        let color_coord = self.color_map[&color];

        // color changed
        if color_coord != self.last_color {
            self.last_color = color_coord;

            // pick color
            desktop.move_cursor(color_coord.x, color_coord.y);
            desktop.left_click(ClickType::Once);

            thread::sleep(Duration::from_millis(self.delay));
        }

        // continue drawing with new color
        desktop.move_cursor(self.xoff + (x * 3) + 1, self.yoff + (y * 3) + 1);
        desktop.left_click(ClickType::Once);

        thread::sleep(Duration::from_millis(self.delay));
    }

    fn draw_internal(&mut self, desktop: &Desktop, image: &Image, map_color: Option<Color>) {
        if let Some(map_color) = &map_color {
            println!("Started drawing color {:?}", map_color);
        }

        if self.checkerboard {
            println!("Drawing first half");
            for y in 0..image.height() {
                let y_even = y % 2 == 0;
                let start = u32::from(y_even as u8);

                for x in (start..image.width()).step_by(2) {
                    let pixel = image.get_pixel(x, y).to_rgb();
                    let color = Color {
                        r: pixel[0],
                        g: pixel[1],
                        b: pixel[2],
                    };

                    if let Some(map_color) = &map_color {
                        if map_color == &color {
                            self.draw_color(desktop, color, x, y);
                        }
                    }
                }
            }

            println!("Drawing second half");
            for y in 0..image.height() {
                let y_even = y % 2 == 0;
                let start = u32::from(y_even as u8) + 1;

                for x in (start..image.width()).step_by(2) {
                    let pixel = image.get_pixel(x, y).to_rgb();
                    let color = Color {
                        r: pixel[0],
                        g: pixel[1],
                        b: pixel[2],
                    };

                    if let Some(map_color) = &map_color {
                        if map_color == &color {
                            self.draw_color(desktop, color, x, y);
                        }
                    }
                }
            }
        } else {
            for y in 0..image.height() {
                for x in 0..image.width() {
                    let pixel = image.get_pixel(x, y).to_rgb();
                    let color = Color {
                        r: pixel[0],
                        g: pixel[1],
                        b: pixel[2],
                    };

                    if let Some(map_color) = &map_color {
                        if map_color == &color {
                            self.draw_color(desktop, color, x, y);
                        }
                    }
                }
            }
        }

        println!("Drawing finished");
    }

    pub fn draw(&mut self, desktop: &Desktop, image: &Image) {
        let timeout = self.timeout;
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(timeout));
            println!("Drawing timed out");
            std::process::exit(1);
        });

        if self.batch_colors {
            let mut keys: Vec<_> = self.color_map.keys().cloned().collect();

            // makes sure the darkest colors are drawn first
            keys.sort_by(|a, b| (a.r + a.g + a.b).cmp(&(b.r + b.g + b.b)));

            for map_color in keys {
                if map_color == WHITE {
                    continue;
                }

                self.draw_internal(desktop, image, Some(map_color));
            }
        } else {
            self.draw_internal(desktop, image, None);
        }
    }
}
