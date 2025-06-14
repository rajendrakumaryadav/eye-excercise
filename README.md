# eye-excercise

This repository contains **eye_guard**, a small utility that reminds you to rest your eyes periodically. Every 30 minutes it shows a notification and displays an overlay with a short eye exercise animation.

## How to Run

```bash
cd eye_guard
cargo run
```

The overlay uses `winit` and `pixels` for a minimal animation and `notify-rust` for desktop notifications. Tokio drives the periodic timer.
