fn fix_incorrect_order() {
    cook_order();
    // super keyword allows access to elements outside function module
    super::serve_order();
}

fn cook_order() {}

// struct is public, but only `toast` property is public
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
