use std::{env, error::Error, fs::File, io::BufReader, time::Duration};

use rodio::{Decoder, Source};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Usage: cargo run <path_to_mp3>");
    let file = File::open(input_file)?;
    let source = Decoder::new(BufReader::new(file))?;
    let duration = source.total_duration().unwrap_or(Duration::from_secs(1));
    let total_secs = duration.as_secs_f64();

    println!("Path: {}", input_file);
    println!("Duration: {:?}s", total_secs);

    Ok(())
}
