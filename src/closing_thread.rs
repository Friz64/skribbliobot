use std::{process, thread};
use winit::{ControlFlow, DeviceEvent, Event, EventsLoop, KeyboardInput, VirtualKeyCode};

pub fn start() {
    thread::spawn(move || {
        let mut events_loop = EventsLoop::new();

        events_loop.run_forever(|event| match event {
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Key(KeyboardInput {
                    virtual_keycode, ..
                }) => {
                    if let Some(keycode) = virtual_keycode {
                        if keycode == VirtualKeyCode::Escape {
                            println!("Exiting because the user pressed Escape");
                            process::exit(0);
                        }
                    }

                    ControlFlow::Continue
                }
                _ => ControlFlow::Continue,
            },
            _ => ControlFlow::Continue,
        });
    });

    println!("Close program with Escape");
}
