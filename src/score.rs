use crate::check::snapshot;

pub fn run() -> Result<(), String> {
    let s = snapshot()?;

    println!("-----------------------------");
    println!("  btr - Battery Score");
    println!("-----------------------------");

    // Health component (0-50 points)
    let health_score = (s.health / 100.0 * 50.0) as u32;

    // Wear component (0-20 points) — inverse of wear
    let wear_score = ((100.0 - s.wear) / 100.0 * 20.0) as u32;

    // Cycle count component (0-20 points)
    // 0-200 cycles = full points, degrades linearly to 0 at 1000 cycles
    let cycle_score = match s.cycle_count {
        Some(cycles) => {
            if cycles <= 200 {
                20u32
            } else if cycles >= 1000 {
                0u32
            } else {
                ((1000 - cycles) as f32 / 800.0 * 20.0) as u32
            }
        }
        None => 10, // unknown — give half points
    };

    // Capacity retention component (0-10 points)
    let cap_score = if s.design_cap > 0.0 {
        ((s.full_cap / s.design_cap) * 10.0) as u32
    } else {
        5
    };

    let total = health_score + wear_score + cycle_score + cap_score;
    let total = total.min(100);

    let rating = match total {
        90..=100 => "Excellent",
        75..=89  => "Good",
        60..=74  => "Fair",
        40..=59  => "Poor",
        _        => "Replace soon",
    };

    let bar_filled = (total / 5) as usize;
    let bar: String = format!(
        "[{}{}]",
        "#".repeat(bar_filled),
        "-".repeat(20 - bar_filled)
    );

    println!("  Score:   {} {}/100", bar, total);
    println!("  Rating:  {}", rating);
    println!("-----------------------------");
    println!("  Health:     {:.1}%  ({}/50)", s.health, health_score);
    println!("  Wear:       {:.1}%  ({}/20)", s.wear, wear_score);
    match s.cycle_count {
        Some(c) => println!("  Cycles:     {}     ({}/20)", c, cycle_score),
        None    => println!("  Cycles:     unknown ({}/20)", cycle_score),
    }
    println!("  Capacity:   {:.1}/{:.1} Wh ({}/10)", s.full_cap, s.design_cap, cap_score);
    println!("-----------------------------");

    if total < 60 {
        println!("  Warning: battery health is degraded.");
        println!("  Consider a replacement soon.");
        println!("-----------------------------");
    }

    Ok(())
}
