use rdev::{EventType, listen};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

fn main() {
    let key_count: Arc<AtomicU64> = Arc::new(AtomicU64::new(0));
    let listener = Arc::clone(&key_count);

    if let Err(error) = listen(move |event| {
        if let EventType::KeyPress(_) = event.event_type {
            let new = listener.fetch_add(1, Ordering::Relaxed) + 1;

            println!("Key pressed: {}", new);
        }
    }) {
        println!("Error: {:?}", error);
    }
}
