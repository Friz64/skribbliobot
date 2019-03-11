use crate::{
    desktop::Desktop,
    drawer::{Box, Drawer},
    image_converter,
    image_downloader::{self, DownloadImageError, ImageDownloader},
    settings::Settings,
};
use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use glib::{MainContext, Receiver, Sender};
use gtk::{
    prelude::*, Application, ApplicationWindow, Builder, Button, CheckButton, Entry, IconView,
    Label, ListStore, Scale, SearchEntry, StaticType,
};
use image::DynamicImage;
use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};
use uuid::Uuid;

#[derive(Clone)]
enum Instruction {
    UpdateSettings,
    UpdateStatus(String),
    NewImages(Uuid),
    AddImage(Uuid, Vec<u8>),
}

#[derive(Clone)]
struct Message {
    uuid: Option<Uuid>,
    instruction: Instruction,
}

impl Message {
    fn send(sender: Sender<Message>, instruction: Instruction) {
        let message = Message {
            uuid: None,
            instruction,
        };

        sender.send(message).unwrap();
    }

    fn send_waiting(
        sender: Sender<Message>,
        uuid_list: Arc<RwLock<Vec<Uuid>>>,
        instruction: Instruction,
    ) {
        let uuid = Uuid::new_v4();

        let message = Message {
            uuid: Some(uuid),
            instruction,
        };

        sender.send(message).unwrap();

        // wait for the message to be processed
        while !uuid_list.read().unwrap().contains(&uuid) {
            thread::sleep(Duration::from_millis(1));
        }
    }
}

#[derive(Clone)]
pub struct GTK {
    pub application: Application,
    pub window: ApplicationWindow,
    pub drawing_x: Entry,
    pub drawing_y: Entry,
    pub drawing_width: Entry,
    pub drawing_height: Entry,
    pub color_x: Entry,
    pub color_y: Entry,
    pub color_width: Entry,
    pub color_height: Entry,
    pub dither: CheckButton,
    pub checkerboard: CheckButton,
    pub grayscale: CheckButton,
    pub delay: Scale,
    pub scale: Scale,
    pub search: SearchEntry,
    pub images_view: IconView,
    pub images_store: ListStore,
    pub status: Label,
    pub draw: Button,
    pub save: Button,
}

pub struct GUI {
    sender: Sender<Message>,
    uuid_list: Arc<RwLock<Vec<Uuid>>>,
    desktop: Arc<Desktop>,
    drawer_running: Arc<AtomicBool>,
    settings: Arc<RwLock<Settings>>,
    images_list: Arc<RwLock<Vec<DynamicImage>>>,
    gtk: GTK,
}

impl GUI {
    pub fn new(
        settings: io::Result<Settings>,
        desktop: Desktop,
        drawer_running: Arc<AtomicBool>,
    ) -> GUI {
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
        let application = gtk::Application::new("friz64.skribbliobot", Default::default())
            .expect("Failed to initialize GTK Application");

        let glade_src = include_str!("gui.glade");
        let builder = Builder::new_from_string(glade_src);

        let status: Label = builder.get_object("Status").unwrap();
        let settings = match settings {
            Ok(settings) => settings,
            Err(err) => {
                GUI::set_status(status.clone(), &format!("Failed to read settings: {}", err));
                Settings::default()
            }
        };
        let settings = Arc::new(RwLock::new(settings));
        let images_list = Arc::new(RwLock::new(Vec::new()));
        let uuid_list = Arc::new(RwLock::new(Vec::new()));

        let images_store = ListStore::new(&[Pixbuf::static_type()]);
        let images_view: IconView = builder.get_object("ImagesView").unwrap();
        images_view.set_model(&images_store);
        images_view.set_pixbuf_column(0);

        let gtk = GTK {
            application,
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
            search: builder.get_object("Search").unwrap(),
            images_view,
            images_store,
            status,
            draw: builder.get_object("Draw").unwrap(),
            save: builder.get_object("Save").unwrap(),
        };

        GUI::set_receiver(
            receiver,
            settings.clone(),
            images_list.clone(),
            uuid_list.clone(),
            gtk.clone(),
        );

        GUI {
            sender,
            uuid_list,
            desktop: Arc::new(desktop),
            drawer_running,
            settings,
            images_list,
            gtk,
        }
    }

    fn set_triggers(&self) {
        self.gtk.save.connect_clicked({
            let settings = self.settings.clone();
            let sender = self.sender.clone();
            let uuid_list = self.uuid_list.clone();

            move |_| {
                let settings = settings.clone();
                let sender = sender.clone();
                let uuid_list = uuid_list.clone();

                thread::spawn(move || {
                    Message::send_waiting(sender.clone(), uuid_list, Instruction::UpdateSettings);

                    let settings = settings.read().unwrap();
                    if let Err(err) = settings.save() {
                        Message::send(
                            sender.clone(),
                            Instruction::UpdateStatus(format!("Failed to write settings: {}", err)),
                        );
                    };
                });
            }
        });

        self.gtk.draw.connect_clicked({
            let gtk = self.gtk.clone();
            let settings = self.settings.clone();
            let drawer_running = self.drawer_running.clone();
            let images_list = self.images_list.clone();
            let desktop = self.desktop.clone();
            let sender = self.sender.clone();
            let uuid_list = self.uuid_list.clone();

            move |_| {
                let gtk = gtk.clone();
                let settings = settings.clone();
                let drawer_running = drawer_running.clone();
                let images_list = images_list.clone();
                let desktop = desktop.clone();
                let sender = sender.clone();
                let uuid_list = uuid_list.clone();

                let image = gtk
                    .images_view
                    .get_selected_items()
                    .get(0)
                    .map(|tree_path| {
                        let index = images_list.read().unwrap()
                            [tree_path.get_indices()[0] as usize]
                            .clone();
                        Some(index)
                    });

                thread::spawn(move || {
                    Message::send_waiting(sender.clone(), uuid_list, Instruction::UpdateSettings);

                    let settings = settings.read().unwrap();
                    if GUI::is_ready(&settings) {
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

                        let image = image.unwrap_or_else(|| {
                            match image_converter::image_from_clipboard() {
                                Ok(image) => Some(image),
                                Err(err) => {
                                    Message::send(sender.clone(), Instruction::UpdateStatus(err));
                                    None
                                }
                            }
                        });

                        let mut drawer = Drawer::new(
                            drawing_area,
                            color_box,
                            settings.checkerboard,
                            settings.delay as u64,
                        );

                        if let Some(image) = image {
                            let converted = image_converter::convert(
                                image,
                                settings.dither,
                                settings.grayscale.unwrap_or(false),
                                settings.scale,
                                settings.drawing_width,
                                settings.drawing_height,
                            );

                            Message::send(
                                sender.clone(),
                                Instruction::UpdateStatus("Drawing - Cancel with ESC".into()),
                            );

                            drawer_running.store(true, Ordering::Relaxed);
                            drawer.draw(&desktop, &converted, drawer_running.clone());
                            drawer_running.store(false, Ordering::Relaxed);

                            Message::send(sender.clone(), Instruction::UpdateStatus("Idle".into()));
                        }
                    } else {
                        Message::send(
                            sender.clone(),
                            Instruction::UpdateStatus("Please enter positions".into()),
                        );
                    }
                });
            }
        });

        // 6.5 -> 6.5ms
        self.gtk
            .delay
            .connect_format_value(|_, val| format!("{}ms", val));

        // 0.8 -> 80%
        // 0.80000000000000001 -> 80%
        self.gtk
            .scale
            .connect_format_value(|_, val| format!("{}%", (val * 100.0).round()));

        self.gtk.search.connect_activate({
            let images_list = self.images_list.clone();
            let sender = self.sender.clone();
            let uuid_list = self.uuid_list.clone();

            move |search| {
                let images_list = images_list.clone();
                let sender = sender.clone();
                let uuid_list = uuid_list.clone();

                let text = search.get_text().unwrap();
                let text = text.as_str().to_string();

                thread::spawn(move || {
                    images_list.write().unwrap().clear();

                    match ImageDownloader::new(&text) {
                        Ok(mut image_downloader) => {
                            let uuid = Uuid::new_v4();

                            Message::send_waiting(
                                sender.clone(),
                                uuid_list.clone(),
                                Instruction::NewImages(uuid),
                            );

                            loop {
                                match image_downloader.download_image() {
                                    Ok(image) => {
                                        Message::send_waiting(
                                            sender.clone(),
                                            uuid_list.clone(),
                                            Instruction::AddImage(uuid, image),
                                        );
                                    }
                                    Err(err) => match err {
                                        DownloadImageError::Error(err) => Message::send(
                                            sender.clone(),
                                            Instruction::UpdateStatus(err),
                                        ),
                                        DownloadImageError::NoImagesLeft => break,
                                    },
                                }
                            }
                        }
                        Err(err) => Message::send(
                            sender.clone(),
                            Instruction::UpdateStatus(err.to_string()),
                        ),
                    }
                });
            }
        });
    }

    fn is_ready(settings: &Settings) -> bool {
        settings.drawing_x != 0
            && settings.drawing_y != 0
            && settings.drawing_width != 0
            && settings.drawing_height != 0
            && settings.color_x != 0
            && settings.color_y != 0
            && settings.color_width != 0
            && settings.color_height != 0
    }

    fn set_status(label: Label, status: &str) {
        label.set_text(&format!("Status: {}", status));
    }

    fn set_receiver(
        receiver: Receiver<Message>,
        settings: Arc<RwLock<Settings>>,
        images_list: Arc<RwLock<Vec<DynamicImage>>>,
        uuid_list: Arc<RwLock<Vec<Uuid>>>,
        gtk: GTK,
    ) {
        let mut current_image_uuid = Uuid::nil();

        receiver.attach(None, move |msg| {
            let label = gtk.status.clone();

            match msg.instruction {
                Instruction::UpdateSettings => settings.write().unwrap().load_from_gtk(gtk.clone()),
                Instruction::UpdateStatus(status) => GUI::set_status(label, &status),
                Instruction::NewImages(uuid) => {
                    images_list.write().unwrap().clear();
                    gtk.images_store.clear();

                    current_image_uuid = uuid;
                }
                Instruction::AddImage(uuid, data) => {
                    if current_image_uuid == uuid {
                        let image =
                            image::load_from_memory_with_format(&data, image::ImageFormat::JPEG);
                        let pixbuf = image_downloader::pixbuf_from_memory(&data, 0.5);

                        if let (Ok(image), Some(pixbuf)) = (image, pixbuf) {
                            gtk.images_store.insert_with_values(None, &[0], &[&pixbuf]);
                            images_list.write().unwrap().push(image);
                        }
                    }
                }
            };

            if let Some(uuid) = msg.uuid {
                uuid_list.write().unwrap().push(uuid);
            }

            glib::Continue(true)
        });
    }

    pub fn run(self) {
        self.gtk.application.connect_activate({
            self.settings.write().unwrap().save_to_gtk(self.gtk.clone());
            self.set_triggers();

            let window = self.gtk.window.clone();
            move |app| {
                window.set_application(app);
                window.show_all();
            }
        });

        self.gtk.application.run(&[]);
    }
}
