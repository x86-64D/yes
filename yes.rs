use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args_os()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();

    let mut out = if args.is_empty() {
        b"y\n".to_vec()
    } else {
        let mut s = args.join(" ").into_bytes();
        s.push(b'\n');
        s
    };

    let stdout = io::stdout();
    let mut hndl = stdout.lock();

    loop {
        if hndl.write_all(&out).is_err() {
            break;
        }
    }
}
