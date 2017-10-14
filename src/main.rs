use std::env;
use std::fs::OpenOptions;
use std::path::PathBuf;
use chrono::prelude::*;
use std::collections::HashMap;
use std::cmp::Ordering;

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
    exercises: HashMap<String, Exercise>,
    last_updated: DateTime<Local>
}

impl Regimen {
    fn add(&mut self, exer: &str, des: &str) {
        self.exercises.insert(String::from(exer), Exercise { desired: des.parse::<u32>().unwrap(), current: 0, created: Local::now()});
        println!("Added new exercise {}, with a goal for {} per day.", exer, des);
    }

    fn list(&self) {
        for (name, details) in &self.exercises {
            println!("{} {}/{}", name, details.current, details.desired);
        }
    }

    fn done(&mut self, exer: &str, des: &str) {
        let mut exercise = match self.exercises.get_mut(exer) {
            Some(x) => x,
            None => panic!("{} is not a valid exercise!", exer)
        };
        let exercises_done: u32 = match des.parse::<u32>() {
            Ok(x) => x,
            Err(e) => panic!("{}", e)
        };
        exercise.current += exercises_done;
        print!("{} of {} {} done.", exercise.current, exercise.desired, exer);
        match exercise.current.cmp(&exercise.desired) {
            Ordering::Less => println!("Still have {} to go.", exercise.desired - exercise.current),
            Ordering::Equal => println!("Exercise complete!"),
            Ordering::Greater => println!("That's {} more than needed!", exercise.current - exercise.desired)
        }
    }

    fn delete(&mut self, exer: &str) {
        if self.exercises.contains_key(exer)
        {
            self.exercises.remove(exer);
            println!("Deleted {}. Was it too hard for you?", exer);
        } else {
            panic!("No exercise {} to delete!", exer);
        }
    }

    fn goal(&mut self, exer: &str, new_goal: &str) {
        match self.exercises.get_mut(exer) {
            Some(exercise) => {
                exercise.desired = match new_goal.parse::<u32>() {
                    Ok(x) => x,
                    Err(err) => panic!("{}", err)
                };
                println!("Set goal of {} to {}", exer, new_goal);
            },
            None => panic!("{} is not a stored exercise!")
        }
    }

    fn reset_counts(&mut self) {
        for (_, details) in &mut self.exercises {
            details.current = 0;
        }
    }
}

fn main() {
    let file = get_file_read()
        .expect("Could not open file!");

    let mut reg: Regimen = serde_json::from_reader(file).unwrap();

    let mut args = env::args();
    args.next().expect("Only one argument???");


    // If it's not the same day, reset everything
    if reg.last_updated.date() != Local::now().date()
    {
        reg.reset_counts();
    }

    reg.last_updated = Local::now();

    match args.next().expect("TODO proper error").as_ref() {
        "add" => reg.add(args.next().expect("Need exercise name").as_ref(), args.next().expect("Need goal number of reps").as_ref()),
        "list" => reg.list(),
        "done" => reg.done(args.next().expect("Need exercise name").as_ref(), args.next().expect("Need completed number of reps").as_ref()),
        "delete" => reg.delete(args.next().expect("Need exercise name").as_ref()),
        "goal" => reg.goal(args.next().expect("Need exercise name").as_ref(), args.next().expect("Need new goal").as_ref()),
        _ => panic!("Not a valid command")
    };

    let file = get_file_write()
        .expect("Could not open or create file!");

    serde_json::to_writer(file, &reg)
        .expect("Could not write to file!");
}

fn get_file_read () -> std::result::Result<std::fs::File, std::io::Error> {
    let mut config = PathBuf::from(env::home_dir().unwrap());
    config.push(".swole.json");

    OpenOptions::new()
        .read(true)
        .open(config)
}

fn get_file_write () -> std::result::Result<std::fs::File, std::io::Error> {
    let mut config = PathBuf::from(env::home_dir().unwrap());
    config.push(".swole.json");

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config)
}
