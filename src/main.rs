extern crate rand;

use std::io;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let mut useless = String::new();
    let mut used: Vec<i32> = Vec::new();
    let between = Range::new(1, 91);
    let mut rng = rand::thread_rng();
    while used.len() < 90 {
        let mut guessed: i32;
        loop {
            guessed = between.ind_sample(&mut rng);
            if !used.contains(&guessed) { break; }
        }
        used.push(guessed);
        println!("{}", guessed);
        io::stdin().read_line(&mut useless)
            .expect("Failed to read line");
    }
}
