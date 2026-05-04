use battery::Manager;
use sysinfo::{ComponentExt, System, SystemExt};
use crate::color;

fn to_fahrenheit(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}

pub fn run(fahrenheit: bool) -> Result<(), String> {
    let unit = if fahrenheit { "F" } else { "C" };

    let manager = Manager::new().map_err(|e| format!("Failed to access battery manager: {}", e))?;
    let mut batteries = manager
        .batteries()
        .map_err(|e| format!("Failed to list batteries: {}", e))?;

    let battery = batteries
        .next()
        .ok_or("No battery found on this system")?
        .map_err(|e| format!("Failed to read battery: {}", e))?;

    let voltage = battery.voltage().value;
    let watts = battery.energy_rate().value;
    let current = if voltage > 0.1 { watts / voltage } else { 0.0 };
    let battery_temp_c = battery.temperature().map(|t| t.value - 273.15);

    let mut sys = System::new_all();
    sys.refresh_components_list();
    sys.refresh_components();

    let cpu_temp_c = sys.components()
        .iter()
        .find(|c| {
            let label = c.label().to_lowercase();
            label.contains("cpu") || label.contains("core") || label.contains("tctl")
        })
        .map(|c| c.temperature());

    color::dim("-----------------------------");
    color::info("  btr - Thermal & Electrical");
    color::dim("-----------------------------");

    match cpu_temp_c {
        Some(t) => {
            let display = if fahrenheit { to_fahrenheit(t) } else { t };
            print!("  CPU temp:    ");
            color::println_colored(&format!("{:.1}°{}", display, unit), color::temp_color(t));
        }
        None => color::dim("  CPU temp:    unavailable"),
    }

    match battery_temp_c {
        Some(t) => {
            let display = if fahrenheit { to_fahrenheit(t) } else { t };
            print!("  Bat temp:    ");
            color::println_colored(&format!("{:.1}°{}", display, unit), color::temp_color(t));
        }
        None => color::dim("  Bat temp:    unavailable"),
    }

    color::dim("-----------------------------");

    if voltage > 0.1 {
        println!("  Voltage:     {:.3} V", voltage);
    } else {
        color::dim("  Voltage:     unavailable");
    }

    if watts > 0.01 {
        print!("  Power draw:  ");
        color::println_colored(&format!("{:.2} W", watts), color::watts_color(watts));
        println!("  Current:     {:.3} A (derived)", current);
    } else {
        color::dim("  Power draw:  unavailable");
        color::dim("  Current:     unavailable");
    }

    color::dim("-----------------------------");
    color::dim("  Note: if readings seem wrong,");
    color::dim("  unplug charger and run again");
    color::dim("  for proper electrical readings");
    color::dim("-----------------------------");

    Ok(())
}
