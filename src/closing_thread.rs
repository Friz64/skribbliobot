use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};
use winit::{
    event::{DeviceEvent, Event, KeyboardInput, StartCause, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::EventLoopExtUnix,
};

pub fn start(running: Arc<AtomicBool>) {
    thread::spawn(move || {
        let event_loop = EventLoop::<()>::new_any_thread();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(StartCause::Init) => *control_flow = ControlFlow::Wait,
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Key(KeyboardInput {
                    virtual_keycode, ..
                }) => {
                    if let Some(keycode) = virtual_keycode {
                        if keycode == VirtualKeyCode::Escape {
                            running.store(false, Ordering::Relaxed);
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        });
    });
}
