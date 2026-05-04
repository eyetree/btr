use crate::check::snapshot;
use crate::color;

const LOW_BATTERY_PCT: f32 = 20.0;
const HIGH_WEAR_PCT: f32 = 30.0;
const HIGH_POWER_DRAW_W: f32 = 60.0;

pub fn run() -> Result<(), String> {
    let s = snapshot()?;
    let mut warnings: Vec<String> = vec![];

    if s.percentage <= LOW_BATTERY_PCT && s.status == "Discharging" {
        warnings.push(format!(
            "LOW BATTERY: {:.1}% remaining (threshold: {}%)",
            s.percentage, LOW_BATTERY_PCT
        ));
    }

    if s.wear >= HIGH_WEAR_PCT {
        warnings.push(format!(
            "HIGH WEAR: battery degraded by {:.1}% (threshold: {}%)",
            s.wear, HIGH_WEAR_PCT
        ));
    }

    if s.watts >= HIGH_POWER_DRAW_W && s.status == "Discharging" {
        warnings.push(format!(
            "HIGH POWER DRAW: {:.1}W (threshold: {}W)",
            s.watts, HIGH_POWER_DRAW_W
        ));
    }

    color::dim("-----------------------------");
    color::info("  btr - Smart Alerts");
    color::dim("-----------------------------");

    print!("  Charge:    ");
    color::println_colored(&format!("{:.1}%", s.percentage), color::charge_color(s.percentage));
    print!("  Status:    ");
    println!("{}", s.status);
    print!("  Wear:      ");
    color::println_colored(&format!("{:.1}%", s.wear), color::wear_color(s.wear));
    if s.watts > 0.0 {
        print!("  Power:     ");
        color::println_colored(&format!("{:.1}W", s.watts), color::watts_color(s.watts));
    }

    color::dim("-----------------------------");

    if warnings.is_empty() {
        color::good("  All OK -- no alerts triggered.");
        color::dim("-----------------------------");
        std::process::exit(0);
    } else {
        color::bad(&format!("  {} alert(s) triggered:", warnings.len()));
        println!();
        for w in &warnings {
            color::bad(&format!("  [!] {}", w));
        }
        color::dim("-----------------------------");
        std::process::exit(1);
    }
}
