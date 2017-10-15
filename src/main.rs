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

const FILE_NAME: &'static str = ".swole.json";


#[derive(Serialize, Deserialize)]
struct Exercise {
    history: Vec<u32>,
    desired: u32,
    created: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
struct Regimen {
    exercises: HashMap<String, Exercise>,
    last_updated: DateTime<Local>
}

impl Regimen {
    pub fn new() -> Regimen {
        Regimen {
            exercises: HashMap::new(),
            last_updated: Local::now()
        }
    }

    fn add(&mut self, exer: &str, des: &str) {
        if self.exercises.contains_key(exer) {
            panic!("This exercise has already been added!");
        }
        self.exercises.insert(String::from(exer), Exercise { desired: des.parse::<u32>().unwrap(), history: vec![0], created: Local::now()});
        println!("Added new exercise {}, with a goal for {} per day.", exer, des);
    }

    fn list(&self) {
        for (name, details) in &self.exercises {
            println!("{} {}/{}", name, *details.history.last().unwrap(), details.desired);
        }
        if self.exercises.is_empty() {
            println!("No exercises yet!");
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
        *exercise.history.last_mut().unwrap() += exercises_done;
        print!("{} of {} {} done.", exercise.history.last().unwrap(), exercise.desired, exer);
        match exercise.history.last().unwrap().cmp(&exercise.desired) {
            Ordering::Less => println!("Still have {} to go.", exercise.desired - exercise.history.last().unwrap()),
            Ordering::Equal => println!("Exercise complete!"),
            Ordering::Greater => println!("That's {} more than needed!", exercise.history.last().unwrap() - exercise.desired)
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

    fn update_history(&mut self) {
        self.last_updated = Local::now();

        for (_, details) in &mut self.exercises {
            let missed_days: i64 = (self.last_updated.date().signed_duration_since(details.created.date())).num_days() + 1 - details.history.len() as i64;
            for _ in 0..missed_days {
                details.history.push(0);
            }
        }
    }
}

fn main() {
    let mut reg: Regimen = match get_file_read() {
        Ok(f) => serde_json::from_reader(f).unwrap(),
        Err(_) => {
            println!("Can't get {}. Creating a new one.", FILE_NAME);
            Regimen::new()
        }
    };

    reg.update_history();

    let mut args = env::args();
    args.next().expect("Only one argument???");

    match args.next().expect("No command provided").as_ref() {
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
    config.push(FILE_NAME);

    OpenOptions::new()
        .read(true)
        .open(config)
}

fn get_file_write () -> std::result::Result<std::fs::File, std::io::Error> {
    let mut config = PathBuf::from(env::home_dir().unwrap());
    config.push(FILE_NAME);

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(config)
}
