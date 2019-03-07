use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};
use winit::{ControlFlow, DeviceEvent, Event, EventsLoop, KeyboardInput, VirtualKeyCode};

pub fn start(running: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut events_loop = EventsLoop::new();

        events_loop.run_forever(|event| match event {
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Key(KeyboardInput {
                    virtual_keycode, ..
                }) => {
                    if let Some(keycode) = virtual_keycode {
                        if keycode == VirtualKeyCode::Escape {
                            running.store(false, Ordering::Relaxed);
                        }
                    }

                    ControlFlow::Continue
                }
                _ => ControlFlow::Continue,
            },
            _ => ControlFlow::Continue,
        });
    });
}
