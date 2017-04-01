extern crate rustc_serialize;
extern crate crypto;

use std::env;
use std::collections::HashMap;

mod lib;

mod challenge1;
mod challenge2;
mod challenge3;
mod challenge4;
mod challenge5;
mod challenge6;
mod challenge7;
mod challenge8;
mod challenge9;
mod challenge10;

fn main() {
    let mut args = env::args();
    let challenges_map = get_challenges_map();

    // First arg is folder name
    args.next();

    match args.next() {
        Some(str) => match str.parse::<i32>() {
            Ok(i) => match challenges_map.get(&i) {
                Some(fun) => fun(),
                _ => println!("Can't find Challenge # {}", i)
            },
            _ => println!("Can't find Challenge {}", str)
        },
        _ => run_all(challenges_map)
    }

}

fn run_all(challenges_map: HashMap<i32, fn()>) {

    let mut keys = challenges_map.keys().collect::<Vec<_>>();
    keys.sort();

    for key in keys {
        challenges_map.get(key).unwrap()();
    }

}

fn get_challenges_map() -> HashMap<i32, fn()> {
    let mut challenges_map: HashMap<i32, fn()> = HashMap::new();

    challenges_map.insert(1, challenge1::run);
    challenges_map.insert(2, challenge2::run);
    challenges_map.insert(3, challenge3::run);
    challenges_map.insert(4, challenge4::run);
    challenges_map.insert(5, challenge5::run);
    challenges_map.insert(6, challenge6::run);
    challenges_map.insert(7, challenge7::run);
    challenges_map.insert(8, challenge8::run);
    challenges_map.insert(9, challenge9::run);
    challenges_map.insert(10, challenge10::run);

    challenges_map
}
