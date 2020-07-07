
// Chapter 3 of The Rust Programming Language book (https://doc.rust-lang.org/stable/book/title-page.html)
// Common Programming Language Concepts
fn main() {
    variables();
    data_types();
}

// 3.1 Variables and mutability
fn variables() {
    let y = 5; // immutable variable
    let mut x = 5; // mutable variable
    const MAX_POINTS: u32 = 100_000; // constant

    println!("The value of x: {} and y: {}", x, y);
    x = MAX_POINTS; 
    println!("The value of x is: {}", x);

    // Shadowing
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);

    let spaces = "    ";
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len(); // Shadowing can change of data type (can't be done on mut vars)
    println!("The value of spaces is: {}", spaces);
}

fn data_types() {
    let _guess: u32 = "42".parse().expect("Not a number!"); // Explicit type needed

    // Integer representations
    let _dec = 98_222;
    let _hex = 0xFF;
    let _oct = 0o77;
    let _bin = 0b1111_0000;
    let _byte = b'A';

    //let _overflow: u8 = "256".parse().expect("You're a little extra!");

    let _heart_eyed_cat = '😻'; // unicode

    let _tup: (i32, f64, u8) = (500, 6.4, 1); //tuple? more like coolple
    println!("first item in tuple is {}", _tup.0);

    let _arr: [u32; 5]; // array with type and size
    let _arr = [3;5]; // init array with 5 items with value 3
    let _first = _arr[0]; // 0-based because we're sane
    let index = 6;
    println!("Over the edge: {}", _arr[index]);
}
