use std::{
    env,
    fs::File,
    io::{self, BufReader},
    time::Duration,
};

use color_eyre::eyre::Result;

use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    DefaultTerminal, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use rodio::{Decoder, Source};

#[derive(Debug)]
enum SelectedHandle {
    Left,
    PlayHead,
    Right,
}

struct Tui {
    terminal: DefaultTerminal,
}

impl Tui {
    fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        Ok(Self { terminal })
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = io::stdout().execute(LeaveAlternateScreen);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Usage: cargo run <path_to_mp3>");
    let file = File::open(input_file)?;
    let source = Decoder::new(BufReader::new(file))?;
    let duration = source.total_duration().unwrap_or(Duration::from_secs(1));
    let total_secs = duration.as_secs_f64();

    println!("Path: {}", input_file);
    println!("Duration: {:?}s", total_secs);

    let handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("Failed to open audio device");

    let file_to_play = File::open(input_file)?;
    let mut player = rodio::play(&handle.mixer(), file_to_play)?;

    player.pause();

    player.set_volume(0.1);

    let mut tui = Tui::new()?;
    let mut is_playing = false;
    let mut volume: f32 = 0.5;

    loop {
        tui.terminal.draw(|f| {
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(10),   // Waveform
                    Constraint::Length(3), // Footer / Controls
                ])
                .split(f.area());

            let status = if is_playing { "PLAYING" } else { "PAUSED " };
            let status_color = if is_playing {
                Color::Green
            } else {
                Color::Yellow
            };

            let header = Paragraph::new(vec![Line::from(vec![
                Span::styled(
                    " CUTERS ",
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                ),
                Span::raw(format!("  File: {}  ", input_file)),
                Span::styled(format!("[{}]", status), Style::default().fg(status_color)),
                Span::raw(format!("  Vol: {:.0}%", volume * 100.0)),
            ])])
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(header, main_chunks[0]);

            let display_text = Paragraph::new(format!("File: {}", input_file))
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(display_text, main_chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Char('j') => {
                            volume = (volume - 0.05).max(0.0);
                            player.set_volume(volume);
                        }
                        KeyCode::Char('k') => {
                            volume = (volume + 0.05).min(2.0);
                            player.set_volume(volume);
                        }
                        KeyCode::Char('k') => {
                            todo!()
                        }
                        KeyCode::Char('l') => {
                            todo!()
                        }
                        KeyCode::Char(' ') => {
                            if is_playing {
                                player.pause();
                                is_playing = false;
                            } else {
                                player.play();
                                is_playing = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        };
    }

    drop(tui);

    Ok(())
}
