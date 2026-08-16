use enigo::{Enigo, Key, Keyboard, Settings};
use rdev::{listen, Event, EventType, Key as RdevKey};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Numpad Key Mapper - Press numpad keys (1-9) to trigger Super+number");
    println!("Press Ctrl+C to exit\n");

    let pressed_keys: Arc<Mutex<HashSet<RdevKey>>> = Arc::new(Mutex::new(HashSet::new()));
    let pressed = pressed_keys.clone();

    thread::spawn(move || {
        listen(move |event: Event| {
            let mut keys = pressed.lock().unwrap();
            match event.event_type {
                EventType::KeyPress(key) => {
                    keys.insert(key);
                }
                EventType::KeyRelease(key) => {
                    keys.remove(&key);
                }
                _ => {}
            }
        })
        .expect("Failed to start key listener");
    });

    let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create enigo instance");

    let numpad_mappings = [
        (RdevKey::Kp1, '1'),
        (RdevKey::Kp2, '2'),
        (RdevKey::Kp3, '3'),
        (RdevKey::Kp4, '4'),
        (RdevKey::Kp5, '5'),
        (RdevKey::Kp6, '6'),
        (RdevKey::Kp7, '7'),
        (RdevKey::Kp8, '8'),
        (RdevKey::Kp9, '9'),
    ];

    let mut handled_keys: HashSet<RdevKey> = HashSet::new();

    loop {
        let keys = pressed_keys.lock().unwrap().clone();

        for (numpad_key, num_char) in &numpad_mappings {
            let is_pressed = keys.contains(numpad_key);
            let was_handled = handled_keys.contains(numpad_key);

            if is_pressed && !was_handled {
                println!(
                    "Detected Numpad {} -> sending Super + {}",
                    num_char, num_char
                );

                enigo.key(Key::Meta, enigo::Direction::Press).unwrap();
                enigo
                    .key(Key::Unicode(*num_char), enigo::Direction::Press)
                    .unwrap();
                thread::sleep(Duration::from_millis(20));
                enigo
                    .key(Key::Unicode(*num_char), enigo::Direction::Release)
                    .unwrap();
                enigo
                    .key(Key::Meta, enigo::Direction::Release)
                    .unwrap();

                handled_keys.insert(*numpad_key);
            } else if !is_pressed && was_handled {
                handled_keys.remove(numpad_key);
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}
