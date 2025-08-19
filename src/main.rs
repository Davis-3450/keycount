use crossterm::{
    cursor::MoveToColumn,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use rdev::{EventType, listen};
use std::io::Write;
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

            let mut out = std::io::stdout();
            let _ = execute!(
                out,
                MoveToColumn(0),
                Clear(ClearType::CurrentLine),
                Print(format!("Key pressed: {}", new))
            );
            let _ = out.flush();
        }
    }) {
        println!("Error: {:?}", error);
    }
}
