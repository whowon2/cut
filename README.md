# cut

A terminal-based audio trimmer with a live TUI interface. Load an audio file, set start/end markers, and trim — all without leaving your terminal.

## Features

- 🎵 Play/pause audio playback directly in the terminal
- ✂️ Set start and end trim markers with keyboard controls
- 📍 Navigate a playhead to preview trim regions
- 🔊 Adjust volume on the fly
- 🖥️ Clean TUI interface built with [Ratatui](https://ratatui.rs/)

## Installation

Requires [Rust](https://rustup.rs/) (edition 2024).

```bash
git clone https://github.com/your-username/cut
cd cut
cargo build --release
```

The binary will be at `./target/release/cut`.

## Usage

```bash
cargo run -- <path_to_audio_file>
# or after building:
./target/release/cut <path_to_audio_file>
```

**Example:**

```bash
cut song.mp3
```

## Key Bindings

| Key       | Action                        |
|-----------|-------------------------------|
| `Space`   | Play / Pause                  |
| `1 / 2 / 3` | Select handle (Start / Playhead / End) |
| `H / L`   | Move selected handle left / right |
| `J / K`   | Decrease / Increase volume    |
| `Q`       | Quit                          |

## Dependencies

| Crate        | Purpose                              |
|--------------|--------------------------------------|
| `ratatui`    | Terminal UI rendering                |
| `crossterm`  | Cross-platform terminal control      |
| `rodio`      | Audio decoding and playback          |
| `color-eyre` | Ergonomic error handling and reporting |

## License

MIT
