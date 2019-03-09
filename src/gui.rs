use crate::{
    desktop::Desktop,
    drawer::{Box, Drawer},
    image_converter,
    settings::Settings,
};
use gio::prelude::*;
use gtk::{
    prelude::*, Application, ApplicationWindow, Builder, Button, CheckButton, Entry, Label, Scale,
};
use std::{
    cell::RefCell,
    io,
    ops::Deref,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

// This is very very very very very very sketchy
// Desktop and Label both don't implement Send/Sync
// because they contain pointers which are easy to
// duplicate and cause segfaults with, but as there
// is only one copy of them in a mutex, it should
// be fine, if im not mistaken :)
struct SafetyWrapper<T>(T);

unsafe impl<T> Send for SafetyWrapper<T> {}

impl<T> Deref for SafetyWrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub struct GUI {
    settings: RefCell<Settings>,
    drawer_running: Arc<AtomicBool>,
    desktop: Arc<Mutex<SafetyWrapper<Desktop>>>,
    application: Application,
    window: ApplicationWindow,
    drawing_x: Entry,
    drawing_y: Entry,
    drawing_width: Entry,
    drawing_height: Entry,
    color_x: Entry,
    color_y: Entry,
    color_width: Entry,
    color_height: Entry,
    dither: CheckButton,
    checkerboard: CheckButton,
    grayscale: CheckButton,
    delay: Scale,
    scale: Scale,
    status: Arc<Mutex<SafetyWrapper<Label>>>,
    draw: Button,
    save: Button,
}

impl GUI {
    pub fn new(
        settings: io::Result<Settings>,
        desktop: Desktop,
        drawer_running: Arc<AtomicBool>,
    ) -> GUI {
        let application = gtk::Application::new("friz64.skribbliobot", Default::default())
            .expect("Failed to initialize GTK Application");

        let glade_src = include_str!("gui.glade");
        let builder = Builder::new_from_string(glade_src);

        let status = Arc::new(Mutex::new(SafetyWrapper(
            builder.get_object("Status").unwrap(),
        )));
        let settings = match settings {
            Ok(settings) => settings,
            Err(err) => {
                GUI::set_status(&status, &format!("Failed to read settings: {}", err));
                Settings::default()
            }
        };

        GUI {
            settings: RefCell::new(settings),
            application,
            drawer_running,
            desktop: Arc::new(Mutex::new(SafetyWrapper(desktop))),
            window: builder.get_object("Window").unwrap(),
            drawing_x: builder.get_object("DrawingX").unwrap(),
            drawing_y: builder.get_object("DrawingY").unwrap(),
            drawing_width: builder.get_object("DrawingWidth").unwrap(),
            drawing_height: builder.get_object("DrawingHeight").unwrap(),
            color_x: builder.get_object("ColorX").unwrap(),
            color_y: builder.get_object("ColorY").unwrap(),
            color_width: builder.get_object("ColorWidth").unwrap(),
            color_height: builder.get_object("ColorHeight").unwrap(),
            dither: builder.get_object("Dither").unwrap(),
            checkerboard: builder.get_object("Checkerboard").unwrap(),
            grayscale: builder.get_object("Grayscale").unwrap(),
            delay: builder.get_object("Delay").unwrap(),
            scale: builder.get_object("Scale").unwrap(),
            status,
            draw: builder.get_object("Draw").unwrap(),
            save: builder.get_object("Save").unwrap(),
        }
    }

    fn load_values(&self) {
        if self.settings.borrow().drawing_x != 0 {
            self.drawing_x
                .set_text(&self.settings.borrow().drawing_x.to_string());
        }
        if self.settings.borrow().drawing_y != 0 {
            self.drawing_y
                .set_text(&self.settings.borrow().drawing_y.to_string());
        }
        if self.settings.borrow().drawing_width != 0 {
            self.drawing_width
                .set_text(&self.settings.borrow().drawing_width.to_string());
        }
        if self.settings.borrow().drawing_height != 0 {
            self.drawing_height
                .set_text(&self.settings.borrow().drawing_height.to_string());
        }

        if self.settings.borrow().color_x != 0 {
            self.color_x
                .set_text(&self.settings.borrow().color_x.to_string());
        }
        if self.settings.borrow().color_y != 0 {
            self.color_y
                .set_text(&self.settings.borrow().color_y.to_string());
        }
        if self.settings.borrow().color_width != 0 {
            self.color_width
                .set_text(&self.settings.borrow().color_width.to_string());
        }
        if self.settings.borrow().color_height != 0 {
            self.color_height
                .set_text(&self.settings.borrow().color_height.to_string());
        }

        self.delay.set_value(self.settings.borrow().delay);
        self.scale.set_value(self.settings.borrow().scale);

        self.dither.set_active(self.settings.borrow().dither);
        self.checkerboard
            .set_active(self.settings.borrow().checkerboard);
        self.grayscale.set_active(
            self.settings
                .borrow()
                .grayscale
                .unwrap_or_else(|| Settings::default().grayscale.unwrap()),
        );
    }

    fn update_settings(gui: Rc<GUI>) {
        gui.settings.borrow_mut().drawing_x = gui
            .drawing_x
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        gui.settings.borrow_mut().drawing_y = gui
            .drawing_y
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        gui.settings.borrow_mut().drawing_height = gui
            .drawing_height
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        gui.settings.borrow_mut().drawing_width = gui
            .drawing_width
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);

        gui.settings.borrow_mut().color_x = gui
            .color_x
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        gui.settings.borrow_mut().color_y = gui
            .color_y
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        gui.settings.borrow_mut().color_height = gui
            .color_height
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);
        gui.settings.borrow_mut().color_width = gui
            .color_width
            .get_text()
            .unwrap()
            .as_str()
            .parse()
            .unwrap_or(0);

        gui.settings.borrow_mut().delay = gui.delay.get_value();
        gui.settings.borrow_mut().scale = gui.scale.get_value();

        gui.settings.borrow_mut().dither = gui.dither.get_active();
        gui.settings.borrow_mut().checkerboard = gui.checkerboard.get_active();
        gui.settings.borrow_mut().grayscale = Some(gui.grayscale.get_active());
    }

    fn set_status(label: &Mutex<SafetyWrapper<Label>>, status: &str) {
        let lock = label.lock().unwrap();
        lock.set_text(&format!("Status: {}", status));
    }

    fn is_ready(gui: Rc<GUI>) -> bool {
        gui.settings.borrow().drawing_x != 0
            && gui.settings.borrow().drawing_y != 0
            && gui.settings.borrow().drawing_width != 0
            && gui.settings.borrow().drawing_height != 0
            && gui.settings.borrow().color_x != 0
            && gui.settings.borrow().color_y != 0
            && gui.settings.borrow().color_width != 0
            && gui.settings.borrow().color_height != 0
    }

    fn set_triggers(gui: Rc<GUI>) {
        gui.save.connect_clicked({
            let gui = gui.clone();
            move |_| {
                GUI::update_settings(gui.clone());
                if let Err(err) = gui.settings.borrow().save() {
                    GUI::set_status(&gui.status, &format!("Failed to write settings: {}", err));
                };
            }
        });

        gui.draw.connect_clicked({
            let gui = gui.clone();
            let drawer_running = gui.drawer_running.clone();

            move |_| {
                GUI::update_settings(gui.clone());

                let status = gui.status.clone();

                if GUI::is_ready(gui.clone()) {
                    let drawer_running = drawer_running.clone();
                    let settings = gui.settings.borrow().clone();
                    let desktop = gui.desktop.clone();

                    let drawing_area = Box {
                        x: settings.drawing_x,
                        y: settings.drawing_y,
                        width: settings.drawing_width,
                        height: settings.drawing_height,
                    };

                    let color_box = Box {
                        x: settings.color_x,
                        y: settings.color_y,
                        width: settings.color_width,
                        height: settings.color_height,
                    };

                    thread::spawn(move || {
                        let mut drawer = Drawer::new(
                            drawing_area,
                            color_box,
                            settings.checkerboard,
                            settings.delay as u64,
                        );

                        match image_converter::image_from_clipboard() {
                            Ok(image) => {
                                let converted = image_converter::convert(
                                    image,
                                    settings.dither,
                                    settings.grayscale.unwrap_or(false),
                                    settings.scale,
                                    settings.drawing_width,
                                    settings.drawing_height,
                                );

                                GUI::set_status(&status, "Drawing - Cancel with ESC");

                                let desktop_lock = desktop.lock().unwrap();

                                drawer_running.store(true, Ordering::Relaxed);
                                drawer.draw(&desktop_lock, &converted, drawer_running.clone());
                                drawer_running.store(false, Ordering::Relaxed);

                                GUI::set_status(&status, "Idle");
                            }
                            Err(err) => {
                                GUI::set_status(&status, &err.to_string());
                            }
                        };
                    });
                } else {
                    GUI::set_status(&status, "Please enter positions");
                }
            }
        });

        // 6.5 -> 6.5ms
        gui.delay
            .connect_format_value(|_, val| format!("{}ms", val));

        // 0.8 -> 80%
        // 0.80000000000000001 -> 80%
        gui.scale
            .connect_format_value(|_, val| format!("{}%", (val * 100.0).round()));
    }

    pub fn run(self) {
        self.load_values();

        let gui = Rc::new(self);
        GUI::set_triggers(gui.clone());

        gui.clone().application.connect_activate({
            let gui = gui.clone();

            move |app| {
                gui.window.set_application(app);
                gui.window.show_all();
            }
        });

        gui.application.run(&[]);
    }
}
