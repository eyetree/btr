# btr - Battery Diagnostic Tool

A cross platform CLI battery diagnostic tool written in Rust, designed to be lightweight, informative and fast.

## Usage

```bash
btr -C
```

## Install

My tool requires [Rust](https://rustup.rs) to be installed.

```bash
cargo install --git https://github.com/eyetree/btr
```

## Output

```text
btr - Battery Diagnostics
Charge:       87.3%
Status:       Discharging
Health:       94.2%
Design cap:   80 Wh
Current cap:  75 Wh
Wear level:   5.8%
Power draw:   12.40 W
Time left:    4h 23m
CPU temp:     unavailable*
RAM usage:    8.2 / 32.0 GB
```

*CPU temperature requires admin privileges on Windows and may be unavailable on some systems.*

## License

MIT
