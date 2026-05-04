use battery::Manager;

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
    println!("  btr - Battery Info");
    println!("-----------------------------");

    let manufacturer = battery.vendor().unwrap_or("Unknown");
    let model = battery.model().unwrap_or("Unknown");
    let serial = battery.serial_number().unwrap_or("Unknown");

    let technology = match battery.technology() {
        battery::Technology::LithiumIon => "Lithium Ion",
        battery::Technology::LeadAcid => "Lead Acid",
        battery::Technology::LithiumPolymer => "Lithium Polymer",
        battery::Technology::NickelMetalHydride => "Nickel Metal Hydride",
        battery::Technology::NickelCadmium => "Nickel Cadmium",
        battery::Technology::NickelZinc => "Nickel Zinc",
        battery::Technology::LithiumIronPhosphate => "Lithium Iron Phosphate",
        battery::Technology::RechargeableAlkalineManganese => "Rechargeable Alkaline",
        _ => "Unknown",
    };

    let cycle_count = battery.cycle_count()
        .map(|c| c.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    println!("  Manufacturer: {}", manufacturer);
    println!("  Model:        {}", model);
    println!("  Serial:       {}", serial);
    println!("  Technology:   {}", technology);
    println!("  Cycle count:  {}", cycle_count);

    println!("-----------------------------");

    // Build DuckDuckGo search query
    let query = match (manufacturer, model) {
        ("Unknown", "Unknown") => "laptop battery replacement".to_string(),
        (m, "Unknown") => format!("{} laptop battery replacement", m),
        ("Unknown", m) => format!("{} battery replacement", m),
        (mfr, mdl) => format!("{} {} replacement battery", mfr, mdl),
    };

    let encoded = query.replace(' ', "+");
    let url = format!("https://duckduckgo.com/?q={}", encoded);

    println!("  Search for replacement:");
    println!("  {}", url);
    println!("-----------------------------");

    Ok(())
}
