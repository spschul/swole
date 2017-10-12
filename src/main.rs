use std::env;
use std::fs::OpenOptions;
use std::path::PathBuf;
use chrono::prelude::*;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Exercise {
    desired: u32,
    current: u32,
    created: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
struct Regimen {
    exercises: HashMap<String, Exercise>
}

impl Regimen {
    fn add(&mut self, exer: &str, des: &str) {
        self.exercises.insert(String::from(exer), Exercise { desired: des.parse::<u32>().unwrap(), current: 0, created: Local::now()});
    }

    fn list(&self) {
        for (name, details) in &self.exercises {
            println!("{} {}/{}", name, details.current, details.desired);
        }
    }

    fn done(&mut self, exer: &str, des: &str) {
        self.exercises.get_mut(exer).unwrap().current += des.parse::<u32>().unwrap();
    }
}

fn main() {
    let file = get_file()
        .expect("Could not open or create file!");

    let mut reg: Regimen = serde_json::from_reader(file).unwrap();

    let mut args = env::args();
    args.next().expect("Only one argument???");

    match args.next().expect("TODO proper error").as_ref() {
        "add" => reg.add(args.next().expect("Need exercise name").as_ref(), args.next().expect("Need goal number of reps").as_ref()),
        "list" => reg.list(),
        "done" => reg.done(args.next().expect("Need exercise name").as_ref(), args.next().expect("Need completed number of reps").as_ref()),
        _ => println!("Not a valid command")
    };

    let file = get_file()
        .expect("Could not open or create file!");

    serde_json::to_writer(file, &reg)
        .expect("Could not write to file!");
}

fn get_file () -> std::result::Result<std::fs::File, std::io::Error> {
    let mut config = PathBuf::from(env::home_dir().unwrap());
    config.push(".swole.json");

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config)
}
