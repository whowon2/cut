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
    style::{Color, Style},
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

    // let handle =
    //     rodio::DeviceSinkBuilder::open_default_sink().expect("Failed to open audio device");

    // let file_to_play = File::open(input_file)?;
    // let mut player = rodio::play(&handle.mixer(), file_to_play)?;

    // std::thread::sleep(Duration::from_secs(2));

    let mut tui = Tui::new()?;

    loop {
        tui.terminal.draw(|f| {
            let display_text = Paragraph::new(format!("Playing: {}", input_file))
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().title("Mp3 cut").borders(Borders::ALL));

            f.render_widget(display_text, f.area());
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Char('h') => {
                            todo!()
                        }
                        KeyCode::Char('j') => {
                            todo!()
                        }
                        KeyCode::Char('k') => {
                            todo!()
                        }
                        KeyCode::Char('l') => {
                            todo!()
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
