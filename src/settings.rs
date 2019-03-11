use crate::gui::GTK;
use gtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::{self, Read},
};

const FILENAME: &str = "skribbl_settings.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub drawing_x: u32,
    pub drawing_y: u32,
    pub drawing_width: u32,
    pub drawing_height: u32,
    pub color_x: u32,
    pub color_y: u32,
    pub color_width: u32,
    pub color_height: u32,
    pub delay: f64,
    pub scale: f64,
    pub dither: bool,
    pub checkerboard: bool,
    // we need to annotate every new setting with this
    // for it to be able to load old settings
    pub grayscale: Option<bool>,
}

impl Settings {
    pub fn load() -> io::Result<Settings> {
        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(FILENAME)?;

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        Ok(match serde_json::from_str(&content) {
            Ok(settings) => settings,
            Err(_) => {
                let default = Settings::default();
                default.save()?;
                default
            }
        })
    }

    pub fn save(&self) -> io::Result<()> {
        let content = serde_json::to_string(self).unwrap();

        fs::write(FILENAME, &content)
    }

    pub fn save_to_gtk(&mut self, gtk: GTK) {
        if self.drawing_x != 0 {
            gtk.drawing_x.set_text(&self.drawing_x.to_string());
        }
        if self.drawing_y != 0 {
            gtk.drawing_y.set_text(&self.drawing_y.to_string());
        }
        if self.drawing_width != 0 {
            gtk.drawing_width.set_text(&self.drawing_width.to_string());
        }
        if self.drawing_height != 0 {
            gtk.drawing_height
                .set_text(&self.drawing_height.to_string());
        }

        if self.color_x != 0 {
            gtk.color_x.set_text(&self.color_x.to_string());
        }
        if self.color_y != 0 {
            gtk.color_y.set_text(&self.color_y.to_string());
        }
        if self.color_width != 0 {
            gtk.color_width.set_text(&self.color_width.to_string());
        }
        if self.color_height != 0 {
            gtk.color_height.set_text(&self.color_height.to_string());
        }

        gtk.delay.set_value(self.delay);
        gtk.scale.set_value(self.scale);

        gtk.dither.set_active(self.dither);
        gtk.checkerboard.set_active(self.checkerboard);
        gtk.grayscale.set_active(
            self.grayscale
                .unwrap_or_else(|| Settings::default().grayscale.unwrap()),
        );
    }

    pub fn load_from_gtk(&mut self, gtk: GTK) {
        self.drawing_x = gtk
            .drawing_x
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        self.drawing_y = gtk
            .drawing_y
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        self.drawing_height = gtk
            .drawing_height
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        self.drawing_width = gtk
            .drawing_width
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);

        self.color_x = gtk
            .color_x
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        self.color_y = gtk
            .color_y
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        self.color_height = gtk
            .color_height
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        self.color_width = gtk
            .color_width
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);

        self.delay = gtk.delay.get_value();
        self.scale = gtk.scale.get_value();
        self.dither = gtk.dither.get_active();
        self.checkerboard = gtk.checkerboard.get_active();
        self.grayscale = Some(gtk.grayscale.get_active());
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            drawing_x: 0,
            drawing_y: 0,
            drawing_width: 0,
            drawing_height: 0,
            color_x: 0,
            color_y: 0,
            color_width: 0,
            color_height: 0,
            delay: 7.0,
            scale: 1.0,
            dither: true,
            checkerboard: true,
            grayscale: Some(false),
        }
    }
}
