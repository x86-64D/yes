use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let out = if args.is_empty() {
        "y".to_string()
    } else {
        args.join(" ")
    };

    let stdout = io::stdout();
    let mut hndl = stdout.lock();

    loop {
        if writeln!(hndl, "{}", out).is_err() {
            break;
        }
    }
}
