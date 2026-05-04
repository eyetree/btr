# btr - Battery Diagnostic Tool

A lightweight, cross-platform CLI tool for battery diagnostics written in Rust. Fast, private, no telemetry.

## Install

Requires [Rust](https://rustup.rs).

```bash
cargo install --git https://github.com/eyetree/btr
```

## Commands

| Command | Description |
|---------|-------------|
| `btr -C` | Check current battery status |
| `btr -I` | Battery info + DuckDuckGo replacement search |
| `btr -H` | View history log |
| `btr -W` | Live watch mode (default 5s refresh) |
| `btr -W 10` | Live watch mode with custom interval |
| `btr -S` | Battery health score out of 100 |
| `btr -E` | Export snapshot to JSON |
| `btr -E csv` | Export snapshot to CSV |
| `btr -G` | ASCII graph of charge and health over time |
| `btr --version` | Show version |

## Output examples

**`btr -C`**
```text
-----------------------------
  btr - Battery Diagnostics
-----------------------------
  Charge:       87.3%
  Status:       Discharging
  Health:       94.2%
  Design cap:   80.0 Wh
  Current cap:  75.4 Wh
  Wear level:   5.8%
  Power draw:   12.40 W
  Time left:    4h 23m
-----------------------------
  CPU temp:     unavailable*
  RAM usage:    8.2 / 32.0 GB
-----------------------------
```

**`btr -S`**
```text
-----------------------------
  btr - Battery Score
-----------------------------
  Score:   [##################--] 94/100
  Rating:  Excellent
-----------------------------
  Health:     94.2%  (47/50)
  Wear:       5.8%   (19/20)
  Cycles:     80     (20/20)
  Capacity:   75.4/80.0 Wh (8/10)
-----------------------------
```

**`btr -I`**
```text
-----------------------------
  btr - Battery Info
-----------------------------
  Manufacturer: ATL
  Model:        L24N4PC0
  Serial:       2179
  Technology:   Lithium Ion
  Cycle count:  80
-----------------------------
  Search for replacement:
  https://duckduckgo.com/?q=ATL+L24N4PC0+replacement+battery
-----------------------------
```

## Notes

- History is saved to `~/.btr/history.log` on every `-C` run
- Exports are saved to the current working directory
- CPU temperature may be unavailable on Windows without admin privileges
- `-G` requires at least a few `-C` runs to show graph data

## License

MIT
