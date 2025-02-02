use std::io::{stdin, stdout, Write, prelude::*, BufReader};
use std::path::Path;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread};
use std::fs::File;

pub fn get_user_input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Please enter a correct string");
    s
}

pub fn file_exists(path :&str) ->bool {
    Path::new(path).exists()
}

pub fn read_file_to_strvec(path: &str) -> Result<Vec<&str>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let list: Vec<&str> = vec![];

    Ok(list)
}

pub fn catch_sig() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new([SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            std::process::exit(0);
        }
    });

    Ok(())
}
