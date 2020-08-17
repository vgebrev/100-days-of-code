use add_one;

fn main() {
    let num = 20;
    println!(
        "Hello, world! {} plus one is {}",
        num,
        add_one::add_one(num)
    );
}
