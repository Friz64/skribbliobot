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
