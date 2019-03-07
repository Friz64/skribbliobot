mod closing_thread;
mod colors;
mod desktop;
mod drawer;
mod gui;
mod image_converter;
mod settings;

use desktop::Desktop;
use gui::GUI;
use settings::Settings;
use std::sync::{atomic::AtomicBool, Arc};

fn main() {
    let running = Arc::new(AtomicBool::new(false));
    closing_thread::start(running.clone());

    let settings = Settings::load();
    let desktop = Desktop::new();

    let gui = GUI::new(settings, desktop, running);
    gui.run();
}
