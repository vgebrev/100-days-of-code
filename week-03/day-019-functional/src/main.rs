use rand::Rng;
use std::io;
use std::process;
use std::thread;
use std::time::Duration;

mod cacher;

fn main() {
    let mut random = rand::thread_rng();

    let intensity = get_intensity().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let random_number = random.gen_range(0, 7);

    generate_workout(intensity, random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = cacher::Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remeber to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn get_intensity() -> Result<u32, std::num::ParseIntError> {
    println!("Enter intensity:");
    let mut intensity = String::new();

    io::stdin()
        .read_line(&mut intensity)
        .expect("Failed to read line");

    intensity.trim().parse::<u32>()
}
