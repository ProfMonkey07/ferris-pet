#[allow(unused_imports)]
use std::io;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
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
    let json_path = "ferris.json";
    let ferris_json = fs::read_to_string(json_path).expect("something is wrong with the save");
    println!("{ferris_json}");
    println!("Welcome back!");
    let mut currentferris = initialise(0, 0, 5, 0.0, "happy");
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
fn initialise(hunger: i32, boredom: i32, shells: i32, age: f32, emo: &str) -> Ferris {
    return Ferris {
        hunger,
        boredom,
        shells,
        age,
        emotion: match emo {
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