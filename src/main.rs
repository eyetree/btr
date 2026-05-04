use clap::{Parser, Subcommand};

mod check;
mod info;
mod history;
mod watch;
mod score;
mod export;
mod graph;
mod alerts;
mod temps;
mod color;

#[derive(Parser)]
#[command(name = "btr", about = "Battery diagnostic tool", version = "0.6.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(short_flag = 'C', about = "Check battery status")]
    Check {
        #[arg(long = "no-log", help = "Skip saving to history log")]
        no_log: bool,
    },
    #[command(short_flag = 'I', about = "Show battery info and find replacement")]
    Info,
    #[command(short_flag = 'H', about = "Show battery history log")]
    History {
        #[arg(long = "last", default_value = "20", help = "Show last N entries")]
        last: usize,
    },
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
    #[command(short_flag = 'A', about = "Check smart alerts (exit 1 if triggered)")]
    Alerts,
    #[command(short_flag = 'T', about = "Show thermal and electrical readings")]
    Temps {
        #[arg(short = 'f', long = "fahrenheit", help = "Show temperature in Fahrenheit")]
        fahrenheit: bool,
    },
    #[command(about = "Clear battery history log")]
    Clean,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Check { no_log }       => check::run(!no_log),
        Commands::Info                   => info::run(),
        Commands::History { last }       => history::show(last),
        Commands::Watch { interval }     => watch::run(interval),
        Commands::Score                  => score::run(),
        Commands::Export { format }      => export::run(&format),
        Commands::Graph                  => graph::run(),
        Commands::Alerts                 => alerts::run(),
        Commands::Temps { fahrenheit }   => temps::run(fahrenheit),
        Commands::Clean                  => history::clean(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
