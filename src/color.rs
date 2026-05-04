use crossterm::style::{Color, SetForegroundColor, ResetColor};
use crossterm::ExecutableCommand;
use std::io::stdout;

pub fn print_colored(text: &str, color: Color) {
    let mut out = stdout();
    let _ = out.execute(SetForegroundColor(color));
    print!("{}", text);
    let _ = out.execute(ResetColor);
}

pub fn println_colored(text: &str, color: Color) {
    print_colored(text, color);
    println!();
}

// Semantic color helpers
pub fn good(text: &str) { println_colored(text, Color::Green); }
pub fn warn(text: &str) { println_colored(text, Color::Yellow); }
pub fn bad(text: &str)  { println_colored(text, Color::Red); }
pub fn info(text: &str) { println_colored(text, Color::Cyan); }
pub fn dim(text: &str)  { println_colored(text, Color::Grey); }

pub fn charge_color(pct: f32) -> Color {
    if pct > 50.0 { Color::Green }
    else if pct > 20.0 { Color::Yellow }
    else { Color::Red }
}

pub fn health_color(pct: f32) -> Color {
    if pct >= 80.0 { Color::Green }
    else if pct >= 60.0 { Color::Yellow }
    else { Color::Red }
}

pub fn wear_color(pct: f32) -> Color {
    if pct < 20.0 { Color::Green }
    else if pct < 40.0 { Color::Yellow }
    else { Color::Red }
}

pub fn temp_color(c: f32) -> Color {
    if c < 60.0 { Color::Green }
    else if c < 80.0 { Color::Yellow }
    else { Color::Red }
}

pub fn watts_color(w: f32) -> Color {
    if w < 30.0 { Color::Green }
    else if w < 55.0 { Color::Yellow }
    else { Color::Red }
}
