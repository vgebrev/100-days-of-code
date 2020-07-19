// both s1 and s2 must live at least as long as lifetime 'a
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str { 
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// An instance of ImportantExcerpt can't outlive the reference held in its part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

fn main() {
    /*let r;
    {
        let x = 5;
        r = &x; // <- x doesn't live past this scope, compiletime error
    }
    */

    let x = 5;
    let r = &x; // <- x lives past r, so we can use reference 
    println!("r: {}", r);

    // Lifetime annotations in function signatures
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Lifetime annotations in struct definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{} (level {})", i.announce_and_return_part("foo"), i.level());
}
