mod front_of_house;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order() // need to use super to call to parent module function
    }

    fn cook_order() {}

    pub enum Appetizer { // Setting this as pub exposes all the values within it, unlike a pub Struct
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn deliver_order() {}

//use crate::front_of_house::hosting; // Bring front_of_house into this scope so we can call it in eat_at_restaurant()
pub use crate::front_of_house::hosting; // This allows external code paths to call this vs. needing to call front_of_house

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // This next line breaks if we uncomment it, because it's private by default
    // meal.seasonal_fruit = String::from("Blueberries");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist(); // From calling crate into scope above
}





