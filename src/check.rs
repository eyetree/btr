use battery::Manager;
use sysinfo::{ComponentExt, System, SystemExt};
use crate::history;

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

    // .value is in joules — divide by 3600 to get Wh
    let design_cap = battery.energy_full_design().value / 3600.0;
    let full_cap = battery.energy_full().value / 3600.0;
    let wear = 100.0 - health;

    // energy_rate is in watts already
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
        percentage,
        status,
        health,
        design_cap,
        full_cap,
        wear,
        watts,
        cpu_temp,
        used_gb,
        total_gb,
        cycle_count,
    })
}

pub fn print_snapshot(s: &BatterySnapshot) {
    println!("-----------------------------");
    println!("  btr - Battery Diagnostics");
    println!("-----------------------------");
    println!("  Charge:       {:.1}%", s.percentage);
    println!("  Status:       {}", s.status);
    println!("  Health:       {:.1}%", s.health);
    println!("  Design cap:   {:.1} Wh", s.design_cap);
    println!("  Current cap:  {:.1} Wh", s.full_cap);
    println!("  Wear level:   {:.1}%", s.wear);
    if s.watts > 0.0 {
        println!("  Power draw:   {:.2} W", s.watts);
    }
    println!("-----------------------------");
    match s.cpu_temp {
        Some(temp) => println!("  CPU temp:     {:.1}C", temp),
        None => println!("  CPU temp:     unavailable"),
    }
    println!("  RAM usage:    {:.1} / {:.1} GB", s.used_gb, s.total_gb);
    println!("-----------------------------");
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
