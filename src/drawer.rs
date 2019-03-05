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
    delay: u64,
    color_coords: HashMap<Color, ColorCoord>,
    last_color: ColorCoord,
}

impl Drawer {
    pub fn new(xoff: u32, yoff: u32, checkerboard: bool, delay: u64, color_box: Box) -> Drawer {
        Drawer {
            xoff,
            yoff,
            checkerboard,
            delay,
            color_coords: calculate_color_positions(color_box),
            last_color: ColorCoord { x: 0, y: 0 },
        }
    }

    pub fn draw(&mut self, desktop: &Desktop, image: &Image) {
        let mut draw_queue = DrawQueue::new();

        if self.checkerboard {
            for y in 0..image.height() {
                let start = 1 - (y % 2);
                for x in (start..image.width()).step_by(2) {
                    let pixel = image.get_pixel(x, y).to_rgb();
                    let color = Color {
                        r: pixel[0],
                        g: pixel[1],
                        b: pixel[2],
                    };

                    draw_queue.push(DrawInfo { x, y, color });
                }
            }

            draw_queue.draw(desktop, self);

            for y in 0..image.height() {
                let start = y % 2;
                for x in (start..image.width()).step_by(2) {
                    let pixel = image.get_pixel(x, y).to_rgb();
                    let color = Color {
                        r: pixel[0],
                        g: pixel[1],
                        b: pixel[2],
                    };

                    draw_queue.push(DrawInfo { x, y, color });
                }
            }

            draw_queue.draw(desktop, self);
        } else {
            for y in 0..image.height() {
                for x in 0..image.width() {
                    let pixel = image.get_pixel(x, y).to_rgb();
                    let color = Color {
                        r: pixel[0],
                        g: pixel[1],
                        b: pixel[2],
                    };

                    draw_queue.push(DrawInfo { x, y, color });
                }
            }

            draw_queue.draw(desktop, self);
        }

        println!("=> Drawing finished");
    }
}

struct DrawInfo {
    x: u32,
    y: u32,
    color: Color,
}

struct DrawQueue {
    queue: Vec<DrawInfo>,
}

impl DrawQueue {
    fn new() -> DrawQueue {
        DrawQueue { queue: Vec::new() }
    }

    fn push(&mut self, info: DrawInfo) {
        self.queue.insert(0, info);
    }

    fn draw(&mut self, desktop: &Desktop, drawer: &mut Drawer) {
        self.queue.sort_by(|a, b| {
            (b.color.r + b.color.g + b.color.b).cmp(&(a.color.r + a.color.g + a.color.b))
        });

        while let Some(info) = self.queue.pop() {
            // draw
            if info.color == WHITE {
                return;
            }

            let color_coord = drawer.color_coords[&info.color];

            // color changed
            if color_coord != drawer.last_color {
                drawer.last_color = color_coord;

                // pick color
                desktop.move_cursor(color_coord.x, color_coord.y);
                desktop.left_click(ClickType::Once);

                thread::sleep(Duration::from_millis(drawer.delay));
            }

            // continue drawing with new color
            desktop.move_cursor(
                drawer.xoff + (info.x * 3) + 1,
                drawer.yoff + (info.y * 3) + 1,
            );
            desktop.left_click(ClickType::Once);

            thread::sleep(Duration::from_millis(drawer.delay));
        }
    }
}
