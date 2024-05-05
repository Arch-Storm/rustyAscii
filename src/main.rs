use crossterm::{cursor, execute};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;

    let mut temp_line = String::new();
    let mut output_array: Vec<String> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if line.trim().is_empty() {
            output_array.push(temp_line.clone());
            temp_line.clear();
        } else {
            temp_line.push_str(&line);
            temp_line.push('\n');
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    loop {
        for frame in &output_array {
            execute!(handle, cursor::MoveTo(0, 0))?;
            write!(handle, "{}", frame)?;
            handle.flush()?;
            thread::sleep(Duration::from_millis(50));
        }
    }
}
