use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::check::snapshot;

fn write_err(e: std::io::Error) -> String {
    format!("Write error: {}", e)
}

pub fn run(format: &str) -> Result<(), String> {
    let s = snapshot()?;
    let now = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let path: PathBuf = match format {
        "json" => format!("btr_export_{}.json", now).into(),
        "csv"  => format!("btr_export_{}.csv", now).into(),
        _      => return Err(format!("Unknown format '{}'. Use json or csv.", format)),
    };

    let mut file = File::create(&path)
        .map_err(|e| format!("Could not create file: {}", e))?;

    let cpu_temp = s.cpu_temp.map(|t| format!("{:.1}", t)).unwrap_or_else(|| "null".to_string());
    let cycles = s.cycle_count.map(|c| c.to_string()).unwrap_or_else(|| "null".to_string());

    match format {
        "json" => {
            writeln!(file, "{{").map_err(write_err)?;
            writeln!(file, "  \"timestamp\": \"{}\",", timestamp).map_err(write_err)?;
            writeln!(file, "  \"charge_pct\": {:.1},", s.percentage).map_err(write_err)?;
            writeln!(file, "  \"status\": \"{}\",", s.status).map_err(write_err)?;
            writeln!(file, "  \"health_pct\": {:.1},", s.health).map_err(write_err)?;
            writeln!(file, "  \"design_cap_wh\": {:.1},", s.design_cap).map_err(write_err)?;
            writeln!(file, "  \"full_cap_wh\": {:.1},", s.full_cap).map_err(write_err)?;
            writeln!(file, "  \"wear_pct\": {:.1},", s.wear).map_err(write_err)?;
            writeln!(file, "  \"power_draw_w\": {:.2},", s.watts).map_err(write_err)?;
            writeln!(file, "  \"cpu_temp_c\": {},", cpu_temp).map_err(write_err)?;
            writeln!(file, "  \"ram_used_gb\": {:.1},", s.used_gb).map_err(write_err)?;
            writeln!(file, "  \"ram_total_gb\": {:.1},", s.total_gb).map_err(write_err)?;
            writeln!(file, "  \"cycle_count\": {}", cycles).map_err(write_err)?;
            writeln!(file, "}}").map_err(write_err)?;
        }
        "csv" => {
            writeln!(file, "timestamp,charge_pct,status,health_pct,design_cap_wh,full_cap_wh,wear_pct,power_draw_w,cpu_temp_c,ram_used_gb,ram_total_gb,cycle_count").map_err(write_err)?;
            writeln!(
                file,
                "{},{:.1},{},{:.1},{:.1},{:.1},{:.1},{:.2},{},{:.1},{:.1},{}",
                timestamp,
                s.percentage, s.status, s.health,
                s.design_cap, s.full_cap, s.wear,
                s.watts, cpu_temp,
                s.used_gb, s.total_gb, cycles
            ).map_err(write_err)?;
        }
        _ => {}
    }

    println!("-----------------------------");
    println!("  btr - Export");
    println!("-----------------------------");
    println!("  Format:  {}", format.to_uppercase());
    println!("  Saved:   {}", path.display());
    println!("-----------------------------");

    Ok(())
}
