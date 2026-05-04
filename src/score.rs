use crate::check::snapshot;
use crate::color;
use crossterm::style::Color;

pub fn run() -> Result<(), String> {
    let s = snapshot()?;

    let health_score = (s.health / 100.0 * 50.0) as u32;
    let wear_score = ((100.0 - s.wear) / 100.0 * 20.0) as u32;
    let cycle_score = match s.cycle_count {
        Some(cycles) => {
            if cycles <= 200 { 20u32 }
            else if cycles >= 1000 { 0u32 }
            else { ((1000 - cycles) as f32 / 800.0 * 20.0) as u32 }
        }
        None => 10,
    };
    let cap_score = if s.design_cap > 0.0 {
        ((s.full_cap / s.design_cap) * 10.0) as u32
    } else { 5 };

    let total = (health_score + wear_score + cycle_score + cap_score).min(100);

    let (rating, score_color) = match total {
        90..=100 => ("Excellent", Color::Green),
        75..=89  => ("Good",      Color::Green),
        60..=74  => ("Fair",      Color::Yellow),
        40..=59  => ("Poor",      Color::Red),
        _        => ("Replace soon", Color::Red),
    };

    let bar_filled = (total / 5) as usize;
    let bar = format!("[{}{}]", "#".repeat(bar_filled), "-".repeat(20 - bar_filled));

    color::dim("-----------------------------");
    color::info("  btr - Battery Score");
    color::dim("-----------------------------");

    print!("  Score:   {} ", bar);
    color::println_colored(&format!("{}/100", total), score_color);
    print!("  Rating:  ");
    color::println_colored(rating, score_color);

    color::dim("-----------------------------");

    print!("  Health:     {:.1}%  ", s.health);
    color::println_colored(&format!("({}/50)", health_score), color::health_color(s.health));

    print!("  Wear:       {:.1}%  ", s.wear);
    color::println_colored(&format!("({}/20)", wear_score), color::wear_color(s.wear));

    match s.cycle_count {
        Some(c) => println!("  Cycles:     {}     ({}/20)", c, cycle_score),
        None    => println!("  Cycles:     unknown ({}/20)", cycle_score),
    }

    println!("  Capacity:   {:.1}/{:.1} Wh ({}/10)", s.full_cap, s.design_cap, cap_score);

    color::dim("-----------------------------");

    if total < 60 {
        color::bad("  Warning: battery health is degraded.");
        color::bad("  Consider a replacement soon.");
        color::dim("-----------------------------");
    }

    Ok(())
}
