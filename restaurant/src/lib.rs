fn deliver_order() {}

mod back_of_house {
    use std::string;

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // example of using parent module members
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}
pub fn eat_at_restaurant() {
    // crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::summer("Tae tae");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}