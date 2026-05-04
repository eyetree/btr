use std::thread;
use std::time::Duration;
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    execute,
};
use std::io::stdout;
use crate::check::{snapshot, print_snapshot};

pub fn run(interval: u64) -> Result<(), String> {
    println!("Watching battery stats every {}s — press Ctrl+C to stop", interval);
    thread::sleep(Duration::from_millis(500));

    loop {
        let s = snapshot()?;

        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        ).map_err(|e| format!("Terminal error: {}", e))?;

        println!("  Refreshing every {}s — Ctrl+C to stop", interval);
        print_snapshot(&s);

        thread::sleep(Duration::from_secs(interval));
    }
}
