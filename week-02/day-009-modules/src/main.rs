use rand::Rng;
use std::{env, fmt}; // nested path
use std::io::{self, Write}; // brings in std::io and std::io::Write
use std::collections::*; // use glob/* to bring in all public items

use day_009_modules;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("{}", secret_number);
    day_009_modules::eat_at_restaurant();        
}