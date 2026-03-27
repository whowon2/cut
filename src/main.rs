use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Usage: cargo run <path_to_mp3>");

    println!("{}", input_file)
}
