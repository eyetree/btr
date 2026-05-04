use clap::{Parser, Subcommand};

mod check;

#[derive(Parser)]
#[command(name = "btr", about = "Battery diagnostic tool", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(short_flag = 'C', about = "Check battery status")]
    Check,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check => {
            if let Err(e) = check::run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
