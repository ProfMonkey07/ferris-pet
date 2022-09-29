use std::io;
fn main() {
    println!("Welcome back!");
    let ferris_ascii = "
        _~^~^~_
    \\) /  o o  \\ (/
      '_   -   _'
      / '-----' \\";
    let mut ferris_hunger: i32 = 0;
    let mut ferris_boredom: i32 = 0;
    let mut shells: i32 = 5;
    let mut age: i32 = 0;
    println!("{ferris_ascii}");
    while 1 {
        update(&mut ferris_hunger, &mut ferris_boredom, &mut shells, &mut age);
    }
}


fn update(hunger: &mut i32, boredom: &mut i32, shells: &mut i32, age: &mut i32) {

}


