fn main() {
    let s = String::from("Hello, world!");
    println!("{}", first_word(&s));

    let hello = &s[..5]; //0..5
    let world = &s[7..]; //7..12
    println!("{}, {}!", hello, world);

    println!("'{}'", first_word_slice("foo bar"));

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    for n in slice {
        println!("{}", n);
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
