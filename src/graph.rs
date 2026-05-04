use std::fs;
use std::path::PathBuf;

fn history_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    Ok(home.join(".btr").join("history.log"))
}

struct Entry {
    timestamp: String,
    charge: f32,
    health: f32,
}

fn parse_history() -> Result<Vec<Entry>, String> {
    let path = history_path()?;
    if !path.exists() {
        return Ok(vec![]);
    }

    let contents = fs::read_to_string(&path)
        .map_err(|e| format!("Could not read history: {}", e))?;

    let mut entries = vec![];

    for line in contents.lines() {
        let parts: Vec<&str> = line.splitn(2, " | ").collect();
        if parts.len() != 2 { continue; }

        let timestamp = parts[0].to_string();
        let fields = parts[1];

        let charge = fields.split_whitespace()
            .find(|s| s.starts_with("charge="))
            .and_then(|s| s.strip_prefix("charge="))
            .and_then(|s| s.strip_suffix('%'))
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0);

        let health = fields.split_whitespace()
            .find(|s| s.starts_with("health="))
            .and_then(|s| s.strip_prefix("health="))
            .and_then(|s| s.strip_suffix('%'))
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0);

        entries.push(Entry { timestamp, charge, health });
    }

    Ok(entries)
}

fn draw_bar(value: f32, max: f32, width: usize, fill: char) -> String {
    let filled = ((value / max) * width as f32) as usize;
    let filled = filled.min(width);
    format!("{}{}", fill.to_string().repeat(filled), " ".repeat(width - filled))
}

pub fn run() -> Result<(), String> {
    let entries = parse_history()?;

    if entries.is_empty() {
        println!("No history yet. Run `btr -C` a few times first.");
        return Ok(());
    }

    // Show last 20 entries
    let start = if entries.len() > 20 { entries.len() - 20 } else { 0 };
    let shown: Vec<&Entry> = entries[start..].iter().collect();

    println!("-----------------------------");
    println!("  btr - Battery Graph");
    println!("  Last {} entries", shown.len());
    println!("-----------------------------");
    println!("  Charge % over time:");
    println!();

    for e in &shown {
        let bar = draw_bar(e.charge, 100.0, 20, '#');
        let time = if e.timestamp.len() >= 16 { &e.timestamp[5..16] } else { &e.timestamp };
        println!("  {} [{}] {:.0}%", time, bar, e.charge);
    }

    println!();
    println!("-----------------------------");
    println!("  Health % over time:");
    println!();

    for e in &shown {
        let bar = draw_bar(e.health, 100.0, 20, '=');
        let time = if e.timestamp.len() >= 16 { &e.timestamp[5..16] } else { &e.timestamp };
        println!("  {} [{}] {:.1}%", time, bar, e.health);
    }

    println!("-----------------------------");

    // Simple trend
    if shown.len() >= 2 {
        let first_health = shown.first().unwrap().health;
        let last_health = shown.last().unwrap().health;
        let diff = last_health - first_health;
        if diff < -1.0 {
            println!("  Trend: health dropped {:.1}% over this period", diff.abs());
        } else if diff > 1.0 {
            println!("  Trend: health improved {:.1}% over this period", diff);
        } else {
            println!("  Trend: health stable");
        }
        println!("-----------------------------");
    }

    Ok(())
}
