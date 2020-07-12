#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    //etc...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    // Enum match
    let coins: [Coin; 5] = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(State::Alabama),
        Coin::Quarter(State::Alaska),
    ];
    for coin in coins.iter() {
        println!("{}", value_in_cents(coin));
    }

    // Option match
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five plus_one = {:?}. non = {:?}", six, none);

    // Wildcard match
    let some_u8_value = 1u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (),
    }

    // Match specific case
    let some_other_u8_value = Some(0u8);
    match some_other_u8_value {
        Some(0) => println!("Zero"),
        _ => ()
    }
    // ...more concise (if let)
    if let Some(0) = some_other_u8_value {
        println!("Zero");
    }
    // if let with else (equivalent to _ in match)
    let mut count = 0;
    for coin in coins.iter() {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}", state);
        } else {
            count += 1;
        }
    }
    println!("{} non-quarters", count);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None 
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
