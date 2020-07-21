pub mod adder;

fn print_number_return_10(number: i32) -> i32 {
    println!("Input is {}", number);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = print_number_return_10(8);
        assert_eq!(result, 10);
    }

    #[test]
    #[ignore]
    fn it_dont_work() {
        let result = print_number_return_10(4);
        assert_eq!(result, 4);
    }
}
