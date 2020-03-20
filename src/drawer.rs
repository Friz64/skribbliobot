use crate::{
    colors::{Color, ColorCoord, *},
    desktop::{ClickType, Desktop},
    image_converter::Image,
};
use image::Pixel;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

pub struct Box {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

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
    drawing_area: Box,
    checkerboard: bool,
    delay: u64,
    step: f64,
    color_coords: HashMap<Color, ColorCoord>,
    last_color: ColorCoord,
}

impl Drawer {
    pub fn new(
        drawing_area: Box,
        color_box: Box,
        checkerboard: bool,
        delay: u64,
        step: f64,
    ) -> Drawer {
        Drawer {
            drawing_area,
            checkerboard,
            delay,
            step,
            color_coords: calculate_color_positions(color_box),
            last_color: ColorCoord { x: 0, y: 0 },
        }
    }

    pub fn draw(&mut self, desktop: &Desktop, image: &Image, drawer_running: Arc<AtomicBool>) {
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

            draw_queue.draw(desktop, self, drawer_running.clone());

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

            draw_queue.draw(desktop, self, drawer_running.clone());
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

            draw_queue.draw(desktop, self, drawer_running.clone());
        }
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

    fn draw(&mut self, desktop: &Desktop, drawer: &mut Drawer, drawer_running: Arc<AtomicBool>) {
        self.queue
            .sort_by(|a, b| b.color.brightness().cmp(&a.color.brightness()));

        while let Some(info) = self.queue.pop() {
            let running = drawer_running.load(Ordering::Relaxed);
            if !running {
                return;
            }

            if info.color == WHITE {
                continue;
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
            let new_drawing_area_x = ((f64::from(info.x) * drawer.step) + 1.0).round() as u32;
            let new_drawing_area_y = ((f64::from(info.y) * drawer.step) + 1.0).round() as u32;
            if new_drawing_area_x <= drawer.drawing_area.width
                && new_drawing_area_y <= drawer.drawing_area.height
            {
                desktop.move_cursor(
                    drawer.drawing_area.x + new_drawing_area_x,
                    drawer.drawing_area.y + new_drawing_area_y,
                );
                desktop.left_click(ClickType::Once);
            }

            thread::sleep(Duration::from_millis(drawer.delay));
        }
    }
}
