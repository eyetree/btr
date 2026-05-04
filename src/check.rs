use battery::Manager;
use sysinfo::{ComponentExt, System, SystemExt};
use crossterm::style::Color;
use crate::history;
use crate::color;

pub struct BatterySnapshot {
    pub percentage: f32,
    pub status: String,
    pub health: f32,
    pub design_cap: f32,
    pub full_cap: f32,
    pub wear: f32,
    pub watts: f32,
    pub cpu_temp: Option<f32>,
    pub used_gb: f64,
    pub total_gb: f64,
    pub cycle_count: Option<u32>,
}

pub fn snapshot() -> Result<BatterySnapshot, String> {
    let manager = Manager::new().map_err(|e| format!("Failed to access battery manager: {}", e))?;
    let mut batteries = manager
        .batteries()
        .map_err(|e| format!("Failed to list batteries: {}", e))?;

    let battery = batteries
        .next()
        .ok_or("No battery found on this system")?
        .map_err(|e| format!("Failed to read battery: {}", e))?;

    let percentage = battery.state_of_charge().value * 100.0;
    let status = match battery.state() {
        battery::State::Charging => "Charging",
        battery::State::Discharging => "Discharging",
        battery::State::Full => "Full",
        battery::State::Empty => "Empty",
        _ => "Unknown",
    }.to_string();

    let health = battery.state_of_health().value * 100.0;
    let design_cap = battery.energy_full_design().value / 3600.0;
    let full_cap = battery.energy_full().value / 3600.0;
    let wear = 100.0 - health;
    let watts = battery.energy_rate().value;
    let cycle_count = battery.cycle_count();

    let mut sys = System::new_all();
    sys.refresh_memory();
    sys.refresh_components_list();
    sys.refresh_components();

    let cpu_temp = sys.components()
        .iter()
        .find(|c| {
            let label = c.label().to_lowercase();
            label.contains("cpu") || label.contains("core") || label.contains("tctl")
        })
        .map(|c| c.temperature());

    let used_gb = sys.used_memory() as f64 / 1_073_741_824.0;
    let total_gb = sys.total_memory() as f64 / 1_073_741_824.0;

    Ok(BatterySnapshot {
        percentage, status, health, design_cap, full_cap,
        wear, watts, cpu_temp, used_gb, total_gb, cycle_count,
    })
}

pub fn print_snapshot(s: &BatterySnapshot) {
    color::dim("-----------------------------");
    color::info("  btr - Battery Diagnostics");
    color::dim("-----------------------------");

    print!("  Charge:       ");
    color::println_colored(&format!("{:.1}%", s.percentage), color::charge_color(s.percentage));

    let status_color = match s.status.as_str() {
        "Charging" => Color::Green,
        "Discharging" => Color::Yellow,
        "Full" => Color::Cyan,
        _ => Color::Grey,
    };
    print!("  Status:       ");
    color::println_colored(&s.status, status_color);

    print!("  Health:       ");
    color::println_colored(&format!("{:.1}%", s.health), color::health_color(s.health));

    println!("  Design cap:   {:.1} Wh", s.design_cap);
    println!("  Current cap:  {:.1} Wh", s.full_cap);

    print!("  Wear level:   ");
    color::println_colored(&format!("{:.1}%", s.wear), color::wear_color(s.wear));

    if s.watts > 0.0 {
        print!("  Power draw:   ");
        color::println_colored(&format!("{:.2} W", s.watts), color::watts_color(s.watts));
    }

    color::dim("-----------------------------");

    match s.cpu_temp {
        Some(t) => {
            print!("  CPU temp:     ");
            color::println_colored(&format!("{:.1}C", t), color::temp_color(t));
        }
        None => color::dim("  CPU temp:     unavailable"),
    }

    let ram_pct = (s.used_gb / s.total_gb * 100.0) as f32;
    let ram_color = if ram_pct < 60.0 { Color::Green }
                   else if ram_pct < 85.0 { Color::Yellow }
                   else { Color::Red };
    print!("  RAM usage:    ");
    color::println_colored(&format!("{:.1} / {:.1} GB", s.used_gb, s.total_gb), ram_color);

    color::dim("-----------------------------");
}

pub fn run(save_history: bool) -> Result<(), String> {
    let s = snapshot()?;
    print_snapshot(&s);
    if save_history {
        if let Err(e) = history::save(&s) {
            eprintln!("Warning: could not save history: {}", e);
        }
    }
    Ok(())
}
