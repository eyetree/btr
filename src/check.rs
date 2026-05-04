use battery::Manager;
use sysinfo::{ComponentExt, System, SystemExt};

pub fn run() -> Result<(), String> {
    let manager = Manager::new().map_err(|e| format!("Failed to access battery manager: {}", e))?;
    let mut batteries = manager
        .batteries()
        .map_err(|e| format!("Failed to list batteries: {}", e))?;

    let battery = batteries
        .next()
        .ok_or("No battery found on this system")?
        .map_err(|e| format!("Failed to read battery: {}", e))?;

    println!("-----------------------------");
    println!("  btr - Battery Diagnostics");
    println!("-----------------------------");

    let percentage = battery.state_of_charge().value * 100.0;
    println!("  Charge:       {:.1}%", percentage);

    let status = match battery.state() {
        battery::State::Charging => "Charging",
        battery::State::Discharging => "Discharging",
        battery::State::Full => "Full",
        battery::State::Empty => "Empty",
        _ => "Unknown",
    };
    println!("  Status:       {}", status);

    let health = battery.state_of_health().value * 100.0;
    println!("  Health:       {:.1}%", health);

    let design_cap = battery.energy_full_design().value;
    let full_cap = battery.energy_full().value;
    println!("  Design cap:   {:.0} Wh", design_cap / 1000.0);
    println!("  Current cap:  {:.0} Wh", full_cap / 1000.0);

    let wear = 100.0 - health;
    println!("  Wear level:   {:.1}%", wear);

    let watts = battery.energy_rate().value / 1000.0;
    if watts > 0.0 {
        println!("  Power draw:   {:.2} W", watts);
    }

    match battery.state() {
        battery::State::Discharging => {
            if let Some(ttf) = battery.time_to_empty() {
                let mins = ttf.value / 60.0;
                println!("  Time left:    {:.0}h {:.0}m", (mins / 60.0).floor(), mins % 60.0);
            }
        }
        battery::State::Charging => {
            if let Some(ttf) = battery.time_to_full() {
                let mins = ttf.value / 60.0;
                println!("  Full in:      {:.0}h {:.0}m", (mins / 60.0).floor(), mins % 60.0);
            }
        }
        _ => {}
    }

    println!("-----------------------------");

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

    match cpu_temp {
        Some(temp) => println!("  CPU temp:     {:.1}�C", temp),
        None => println!("  CPU temp:     unavailable"),
    }

    let used_gb = sys.used_memory() as f64 / 1_073_741_824.0;
    let total_gb = sys.total_memory() as f64 / 1_073_741_824.0;
    println!("  RAM usage:    {:.1} / {:.1} GB", used_gb, total_gb);

    println!("-----------------------------");

    Ok(())
}
