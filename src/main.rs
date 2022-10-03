#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::io;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
//struct for json save data loading
#[derive(Serialize, Deserialize)]
struct Save {
    hunger: i32,
    boredom: i32,
    shells: i32,
    age: f32,
    emotion: String,
}
//enum to hold possible ferris emotions
enum Emotions {
    Happy,
    Sad,
    Hungry,
    Bored,
    Sleepy,
}
//struct to hold ferris and his attributes
struct Ferris {
    hunger: i32,
    boredom: i32,
    shells: i32,
    age: f32,
    emotion: Emotions,
}

fn main() {
    println!("Welcome back!");
    let mut currentferris = initialise();
    let ferris_ascii = "
        _~^~^~_
    \\) /  o o  \\ (/
      '_   -   _'
      / '-----' \\";

    println!("{ferris_ascii}");
    loop {
        currentferris = update(currentferris, 1, 1, 0, 0.1);
        thread::sleep(Duration::from_millis(10));
    }
}
//this will be used at the beginning to load data from the json save
fn initialise() -> Ferris {
    let json_path = "ferris.json";
    let ferris_json = fs::read_to_string(json_path).expect("something is wrong with the save");
    println!("{ferris_json}");
    let json_read: Save = read_json(&ferris_json);
    return Ferris {
        hunger: json_read.hunger,
        boredom: json_read.boredom,
        shells: json_read.shells,
        age: json_read.age,
        emotion: match json_read.emotion.as_str() {
            "happy" => Emotions::Happy,
            "sad" => Emotions::Sad,
            "hungry" => Emotions::Hungry,
            "bored" => Emotions::Bored,
            "sleepy" => Emotions::Sleepy,
            &_ => Emotions::Happy,
        },
    };
}
//used to update ferris constantly
fn update(state: Ferris, hincrease: i32, bincrease: i32, sincrease: i32, aincrease: f32) -> Ferris {
    return Ferris {
        hunger: state.hunger + hincrease,
        boredom: state.boredom + bincrease,
        shells: state.shells + sincrease,
        age: state.age + aincrease,
        emotion: state.emotion,
    };
}
//got this function from medium
fn read_json(raw_json:&str) -> Save {
    let parsed: Save = serde_json::from_str(raw_json).unwrap();
    return parsed
}