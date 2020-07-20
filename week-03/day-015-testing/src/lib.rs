#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn panic_attack() {
    panic!("Crippling anxiety");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two_four() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn make_it_fail() {
    //     panic!("I've no idea what I'm doin'!");
    // }

    #[test]
    fn larget_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Veli");
        assert!(
            result.contains("Veli"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Crippling anxiety")]
    fn boo() {
        panic_attack();
    }

    #[test]
    fn hail_to_the_thief() -> Result<(), String> {
        if 2 + 2 == 5 {
            Err(String::from("Suddenly... Radiohead!"))
        } else {
            Ok(()) // Computer
        }
    }
}
