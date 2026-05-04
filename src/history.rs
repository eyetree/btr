use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use crate::check::BatterySnapshot;
use crate::color;

fn history_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let dir = home.join(".btr");
    fs::create_dir_all(&dir).map_err(|e| format!("Could not create ~/.btr: {}", e))?;
    Ok(dir.join("history.log"))
}

pub fn save(s: &BatterySnapshot) -> Result<(), String> {
    let path = history_path()?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("Could not open history file: {}", e))?;

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(
        file,
        "{} | charge={:.1}% status={} health={:.1}% wear={:.1}% watts={:.2}W ram={:.1}/{:.1}GB",
        now, s.percentage, s.status, s.health, s.wear, s.watts, s.used_gb, s.total_gb
    ).map_err(|e| format!("Could not write to history: {}", e))?;

    Ok(())
}

pub fn show(last: usize) -> Result<(), String> {
    let path = history_path()?;

    if !path.exists() {
        println!("No history yet. Run `btr -C` to start logging.");
        return Ok(());
    }

    let contents = fs::read_to_string(&path)
        .map_err(|e| format!("Could not read history file: {}", e))?;

    let lines: Vec<&str> = contents.lines().collect();
    let count = if last == 0 { 20 } else { last };
    let start = if lines.len() > count { lines.len() - count } else { 0 };

    color::dim("-----------------------------");
    color::info("  btr - Battery History");
    color::dim(&format!("  {} entries (showing {})", lines.len(), lines.len() - start));
    color::dim("-----------------------------");

    for line in &lines[start..] {
        println!("  {}", line);
    }

    color::dim("-----------------------------");
    color::dim(&format!("  Log: {}", path.display()));
    color::dim("-----------------------------");

    Ok(())
}

pub fn clean() -> Result<(), String> {
    let path = history_path()?;

    if !path.exists() {
        println!("No history log found — nothing to clean.");
        return Ok(());
    }

    fs::remove_file(&path)
        .map_err(|e| format!("Could not delete history: {}", e))?;

    color::dim("-----------------------------");
    color::good("  History log cleared.");
    color::dim("-----------------------------");

    Ok(())
}
