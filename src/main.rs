#[allow(unused_imports)]
use std::io;
use std::thread;
use std::time::Duration;

enum Emotions {
    Happy,
    Sad,
    Hungry,
    Bored,
    Sleepy,
}

struct Ferris {
    hunger: i32,
    boredom: i32,
    shells: i32,
    age: f32,
    emotion: Emotions,
}

fn main() {
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

fn update(state: Ferris, hincrease: i32, bincrease: i32, sincrease: i32, aincrease: f32) -> Ferris {
    return Ferris {
        hunger: state.hunger + hincrease,
        boredom: state.boredom + bincrease,
        shells: state.shells + sincrease,
        age: state.age + aincrease,
        emotion: state.emotion,
    };
}