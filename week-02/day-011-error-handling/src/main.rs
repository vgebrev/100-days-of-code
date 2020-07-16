use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    //unrecoverable_panic();

    //recoverable_result_enum();

    let line = read_from_file("hello.txt").expect("couldn't read");
    println!("Read line: '{}'", line);

    //let f = File::open("404.txt")?; // can't use the ? operator in a function that doesn't return result

    let g = Guess::new(101); // panic, value is not between 1 and 100
    println!("guess={}", g.value());
}

fn read_from_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

fn _read_from_file_ex (file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?; // ? operator returns Err instead of matching
    Ok(s)
}

fn _recoverable_result_enum() {
    // Less concise way of open-or-create using nested matches
    // let f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e)
    //         },
    //         other_error => { 
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     }
    // };

    // More concise way to open-or-create using unwrap_or_else
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let _f2 = File::open("byebye.txt").unwrap(); // unwrap gives File if Ok, or panic if Err
    let _f3 = File::open("byebye.txt").expect("Never gonna give you up!"); // expect allows us to set panic message
}

fn _unrecoverable_panic() {
    let _v = vec![1, 2, 3];
    _v[3]; //panic: index out of bounds 

    panic!("the disco");
}
