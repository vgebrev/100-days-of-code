mod front_of_house;

pub use crate::front_of_house::hosting; // pub use to "re-export" and allow external code to use hosting:: public items 
use self::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist(); //from self/relative path use - avoid not including defining module
    hosting::add_to_waitlist(); //from crate/absolute path use - using path to defining module is the idiomatic way to bring functions into scope (use path::to::mod, then invoke with mod::fn)
}

use std::collections::HashMap; // bringing structs/enums/etc the idiomatic way is to use the full path (use path:to:mod::struct, then reference with struct)

pub fn make_some_hash_browns() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}


// When we have different modules with the same struct name, we can't import both using full path
use std::fmt;
use std::io;

fn fmt_result() -> fmt::Result {
    fmt::Result::Ok(())
}

fn io_result() -> io::Result<()> {
    Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult; //unles we give one an alias with 'as'

fn fmt_result2() -> fmt::Result {
    Result::Ok(())
}

fn io_result2() -> IoResult<String> {
    Ok(String::from("Pablo Honey"))
}
