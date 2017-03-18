extern crate rustc_serialize;
extern crate crypto;

mod challenge1;
mod challenge2;
mod challenge3;
mod challenge4;
mod challenge5;
mod challenge6;
mod challenge7;
mod utils;

fn main() {
    challenge1::run();
    challenge2::run();
    challenge3::run();
    challenge4::run();
    challenge5::run();
    challenge6::run();
    challenge7::run();
}
