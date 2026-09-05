# sysfetch

A neofetch-style system info tool for Windows (and Linux), written in Rust 🦀

Displays your system info with a colored ASCII logo, the way neofetch does on Linux, but built for Windows, which never had one.

## Usage

```
sysfetch
```

Output includes:
- OS name
- Host name
- Kernel version
- CPU
- RAM (total/used)
- Uptime
- Username
- Colored ASCII Windows/Linux logo

## Build from source

```
git clone https://github.com/diegoolivaa/sysfetch.git
cd sysfetch
cargo build --release
```

The compiled binary will be in `target/release/sysfetch.exe`.

## Built with

- [sysinfo](https://crates.io/crates/sysinfo) — reads system data
- [colored](https://crates.io/crates/colored) — terminal colors
- [whoami](https://crates.io/crates/whoami) — username

## Made by

[@diegoolivaa](https://github.com/diegoolivaa)
