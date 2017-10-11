use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut config = PathBuf::from(env::home_dir().unwrap());
    config.push(".swole");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config)
        .expect("Could not open or create file!");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong during the read of the file!");

    println!("Contents: {}", contents);

    let args: Vec<String> =  env::args().collect();

    println!("{:?}", args);
}
