use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc::Sender;

pub enum KeyboardEvent {
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

pub fn start_listening(tx: Sender<KeyboardEvent>) {
    if let Err(error) = listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            let keyboard_event = match key {
                Key::F6 => Some(KeyboardEvent::F6),
                Key::F7 => Some(KeyboardEvent::F7),
                Key::F8 => Some(KeyboardEvent::F8),
                Key::F9 => Some(KeyboardEvent::F9),
                Key::F10 => Some(KeyboardEvent::F10),
                Key::F11 => Some(KeyboardEvent::F11),
                Key::F12 => Some(KeyboardEvent::F12),
                _ => None,
            };
            if let Some(ke) = keyboard_event {
                tx.send(ke).unwrap();
            }
        }
    }) {
        println!("Error listening to keyboard: {:?}", error)
    }
}
