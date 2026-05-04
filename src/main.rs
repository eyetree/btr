use clap::{Parser, Subcommand};

mod check;
mod info;
mod history;
mod watch;
mod score;
mod export;
mod graph;

#[derive(Parser)]
#[command(name = "btr", about = "Battery diagnostic tool", version = "0.4.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(short_flag = 'C', about = "Check battery status")]
    Check,
    #[command(short_flag = 'I', about = "Show battery info and find replacement")]
    Info,
    #[command(short_flag = 'H', about = "Show battery history log")]
    History,
    #[command(short_flag = 'W', about = "Watch battery stats live")]
    Watch {
        #[arg(default_value = "5", help = "Refresh interval in seconds")]
        interval: u64,
    },
    #[command(short_flag = 'S', about = "Show battery health score")]
    Score,
    #[command(short_flag = 'E', about = "Export battery data")]
    Export {
        #[arg(default_value = "json", help = "Export format: json or csv")]
        format: String,
    },
    #[command(short_flag = 'G', about = "Show battery charge/health graph")]
    Graph,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Check        => check::run(true),
        Commands::Info         => info::run(),
        Commands::History      => history::show(),
        Commands::Watch { interval } => watch::run(interval),
        Commands::Score        => score::run(),
        Commands::Export { format } => export::run(&format),
        Commands::Graph        => graph::run(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
