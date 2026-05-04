use battery::Manager;

fn guess_technology(manufacturer: &str, model: &str) -> &'static str {
    let mfr = manufacturer.to_lowercase();
    let mdl = model.to_lowercase();

    // Most modern laptop batteries are Li-Ion or LiPo
    // ATL (Amperex Technology Limited) makes Li-Ion and LiPo cells
    // Common laptop battery model prefixes that indicate Li-Ion
    if mfr.contains("atl") || mfr.contains("amperex") {
        return "Lithium Ion (ATL)";
    }
    if mfr.contains("lgc") || mfr.contains("lg chem") {
        return "Lithium Ion (LG)";
    }
    if mfr.contains("samsung") || mfr.contains("sdl") {
        return "Lithium Ion (Samsung)";
    }
    if mfr.contains("panasonic") || mfr.contains("sanyo") {
        return "Lithium Ion (Panasonic)";
    }
    if mfr.contains("sony") || mfr.contains("murata") {
        return "Lithium Ion (Sony/Murata)";
    }
    if mfr.contains("sunwoda") {
        return "Lithium Ion (Sunwoda)";
    }
    if mfr.contains("celxpert") || mfr.contains("simplo") {
        return "Lithium Ion (Simplo)";
    }

    // Check model name for clues
    if mdl.contains("lipo") || mdl.contains("polymer") {
        return "Lithium Polymer";
    }
    if mdl.contains("life") || mdl.contains("lifepo") {
        return "Lithium Iron Phosphate";
    }

    // Lenovo model prefix patterns (L = Lithium)
    if mdl.starts_with('l') && mdl.len() >= 8 {
        return "Lithium Ion (inferred)";
    }

    // Safe default for modern laptops
    "Lithium Ion (assumed)"
}

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
        _ => guess_technology(manufacturer, model),
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
