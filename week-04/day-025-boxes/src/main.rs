enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // box data stored on heap
    // use box when size is not known at compile time
    // use box when transferring ownership of large amount of data
    // use box when you care about trait implementation, not specific type
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
