#![allow(unused)]

mod algorithms;
mod challenges;
mod google_course;

use rand::Rng;

fn main() {
    let random_string = random_string(10);
    println!("{}", random_string);
}

fn random_string(len: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
