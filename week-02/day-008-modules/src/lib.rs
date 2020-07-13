mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod service {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant(){
    // crate::front_of_house::hosting::add_to_waitlist(); // absolute path
    front_of_house::hosting::add_to_waitlist(); // relative path
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
    
    //meal.seasonal_fruit = String::from("blueberries"); // can't access private fields
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::service::serve_order();
    }

    fn cook_order() {}

    // struct fields can be public or private
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // public enum's variants are also public
    pub enum Appetizer {
        Soup, 
        Salad
    }
}