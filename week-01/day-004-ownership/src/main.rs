fn main() {
    ownership_scenarios();
    references_borrowing_scenarios();
}

// 4.1. Ownership
fn ownership_scenarios() {
    let s1 = String::from("hello");
    // let s2 = s1; // move s1 into s2
    // println!({}, s1); // error - s1 is invalid after it's been moved
    let s2 = s1.clone(); // clone/deep copy s1
    println!("{}, {}", s1, s2);

    take_ownership(s2);
    //s2 is invalid here, as it moved into the function

    let s3 = takes_and_gives_back(s1);
    println!("{}", s3); // s1 moved into fn's some_string and fn's return moved into s3

    let (s4, l) = calculate_length(s3);
    println!("{} has a length of {}", s4, l);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// 4.2 References and Borrowing
fn references_borrowing_scenarios() {
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of {} is {}", s1, len);

    let mut s2 = s1.clone();
    side_effect_shenanigans(&mut s2);
    println!("Look how they massacred my boy: {}", s2);

    // let mut teenage_mutant_ninja_turtle = String::from("Cowabunga!");
    // let michelangelo = &teenage_mutant_ninja_turtle; // Immutable reference is ok
    // let leonardo = &teenage_mutant_ninja_turtle; // Immutable reference is ok
    // let rafaelo = &mut teenage_mutant_ninja_turtle; // Can't borrow as mutable after borrwing as immutable
    // let donatello = &mut teenage_mutant_ninja_turtle; // 2nd muttable borrow in the same scope
    // println!("{} {} {} {}", michelangelo, leonardo, rafaelo, donatello); 
    // ^^^ Compiler error, can't have 2 mutable references in the same scope, or a mutable borrow and an immutable borrow, prevents data reces

    let reference_to_nothing = dangle();
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn side_effect_shenanigans(s: &mut String) {
    s.push_str("what happens in this fn does not stay in this fn");
}

fn dangle() -> &String {
    let s = String::from("Local scope variable");
    &s //Compiler error. Trying to return a reference to variable as it leaves scope/is dropped
    //Solution is to return the string itself, so ownership is moved out to the caller
}