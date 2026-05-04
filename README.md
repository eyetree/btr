# btr - Battery Diagnostic Tool

A lightweight, cross-platform CLI battery diagnostic tool written in Rust. Fast, private, no telemetry.

## Install

Requires [Rust](https://rustup.rs).

```bash
cargo install --git https://github.com/eyetree/btr
```

## Commands

| Command | Description |
|---------|-------------|
| `btr -C` | Check battery status |
| `btr -C --no-log` | Check without saving to history |
| `btr -I` | Battery info + replacement search |
| `btr -S` | Health score out of 100 |
| `btr -T` | Thermal & electrical readings |
| `btr -T -f` | Same, in Fahrenheit |
| `btr -A` | Smart alerts (exits 1 if triggered) |
| `btr -H` | View history log |
| `btr -H --last 5` | View last N entries |
| `btr -W` | Live watch mode (5s default) |
| `btr -W 10` | Live watch, custom interval |
| `btr -E` | Export snapshot to JSON |
| `btr -E csv` | Export snapshot to CSV |
| `btr -G` | ASCII graph of charge & health over time |
| `btr clean` | Clear history log |

## Color output

🟢 Green — good, healthy, normal
🟡 Yellow — warning, worth watching  
🔴 Red — critical, action needed

Applies to: charge %, health %, wear level, power draw, CPU temp, RAM usage, scores and alerts.

## Notes

- History saved to `~/.btr/history.log` on every `-C` run
- Exports saved to `~/.btr/exports/`
- CPU temperature may be unavailable on Windows without admin privileges
- Electrical readings (`-T`) require battery to be discharging
- `-A` exits with code `1` when alerts trigger — use with cron/scripts
- `-G` needs several `-C` runs over time to show meaningful data

## License

MIT
