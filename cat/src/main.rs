use std::env::args;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::process::exit;
use std::io::{self, Write, stdout};

fn main() {
    let arguments: Vec<String> = args().collect();
    let argc = arguments.len();
    if argc < 2 {
        eprintln!("{}: file name not given", arguments[0]);
        exit(1);
    }
    for i in 1..argc {
        if let Err(err) = do_cat(PathBuf::from(&arguments[i])) {
            eprintln!("{}", err);
            exit(1);
        };
    }
    exit(0);
}

fn do_cat(path: PathBuf) -> io::Result<()> {
    let content = match read_to_string(&path) {
        Ok(content) => content,
        Err(error) => return Err(error.into())
    };
    for line in content.lines() {
        stdout().write_all(line.as_bytes())?;
        stdout().write_all(b"\n")?;
    }
    Ok(())
}
