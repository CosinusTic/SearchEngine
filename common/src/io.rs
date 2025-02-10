use std::io::{stdin, stdout, Write, prelude::*, BufReader};
use std::path::Path;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread};
use std::fs::{File, write};
use std::collections::HashSet;

pub fn get_user_input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Please enter a correct string");
    s
}

pub fn file_exists(path :&str) -> bool {
    Path::new(path).exists()
}

pub fn parse_flines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("failed to open file");
    let buf_reader = BufReader::new(file);
    buf_reader
        .lines()
        .filter_map(|l| l.ok())
        .collect()
}

pub fn dump_to_file(path: &str, content: Vec<String>) -> Result<(), Box<dyn Error>> {
    if file_exists(path) {
        for line in content {
            write(path, line).expect("Failed to write to file");
        }
    }
    Ok(())
}

pub fn create_f(path: &str) -> File {
    File::create(path).unwrap()
}

pub fn parse_fcontent(content: &str) -> Vec<&str> {
    content.lines().collect()
}

pub fn catch_sig(file: File, content: &HashSet<String>) {
    let mut signals = Signals::new([SIGINT]).unwrap();
    println!("Length: {}", content.len()); 
    thread::spawn(move || {
        for sig in signals.forever() {
            // Exit current file stream and save WIP
            drop(file);
            println!("Received signal {:?}", sig);
            std::process::exit(0);
        }
    });
}
