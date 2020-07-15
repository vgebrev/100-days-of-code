use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new(); //not inserting, so must explicitly specify type annotation
    v.push(42);
    {
        let v2 = vec![1, 1, 3, 5, 8, 12]; //use macro to instantiate a vector with values
        let third: &i32 = &v2[2];

        println!("Third element of v2 = {}", third);
        match v2.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("No third element"),
        }

        // let hundredth = &v[100]; // Panic at the disco!
        // println!("hundredth = {}", hundredth); 
        let hundredth = v.get(100);
        match hundredth {
            Some(val) => println!("The hundredth element is {}", val),
            None => println!("No such element")
        }
    } // v2 freed here

    let /*mut*/ v3 = vec![1, 2, 3, 4, 5];
    let first = v3.get(0); // immutable borrow
    // v3.push(6); // mutable borrow
    let last = v3.last();
    println!("first={:?}, last={:?}", first, last); // immutable borrow used

    iterating_over_vectors();
    using_enums_for_multiple_types();

    strings();

    hash_maps();
}

fn iterating_over_vectors() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
    }
}

fn using_enums_for_multiple_types() {
    enum SpreadheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadheetCell::Int(3),
        SpreadheetCell::Float(3.1415),
        SpreadheetCell::Text(String::from("Foo bar"))
    ];

    for i in row {
        match i {
            SpreadheetCell::Int(i) => println!("Integer {}", i),
            SpreadheetCell::Float(f) => println!("Float {}", f),
            SpreadheetCell::Text(t) => println!("Text {}", t),
        }
    }
}

fn strings() {
    let mut s = String::new(); // new String
    s.push_str("contents"); // and adding a string value
    let str_literal = "contents";
    let s2 = str_literal.to_string(); // &str to String
    let s3 = "contents".to_string(); // literal to String
    let s4 = String::from("contents"); // new String from literal
    let mut s5 = String::from("content"); // new String from literal
    s.push('s'); // and adding a character

    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7; //s6 has moved and can't be used after, but s7 is borrowed, so can
    println!("Hello, {} {}", s7, s8);

    let s9 = format!("{} {}", s8, s7); // use format macro to concat strings
}

fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    let keys = vec!["foo", "bar"];
    let values = vec![42, 69];
    // yikes
    let map: HashMap<_,_> = keys.into_iter().zip(values.into_iter()).collect();
    
    // get value with get
    let v = map.get("foo");
    match v {
        Some(v) => println!("{}", v),
        None => println!("No key")
    };

    // iterate key-value pairs
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    let mut map2 = HashMap::new();
    map2.insert("foo", 42);
    map2.insert("foo", 69); //overwrite value for key "foo"
    let bar_ref = map2.entry("bar").or_insert(420); // check if key "bar" exists and insert if not
    *bar_ref = 42;
    println!("{:?}", map2);

}