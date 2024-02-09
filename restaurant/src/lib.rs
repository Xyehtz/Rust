// We can import libraries like this
// use rand::Rng;
// use rand::CryptoRng;
// use rand::Fill;

// But if we know that we are going to import different things from the same library we can do this
use rand::{
    Rng,
    CryptoRng,
    Fill
};

// This is a module, they are declared with the mod keyword and the name of the module
mod front_of_the_house {
    pub mod hosting { // This is a module with functions nested inside of it
        pub fn add_to_wait_list() {
            //
        }

        fn seat_at_table() {
            //
        }
    }

    mod serving { // This is the same example as hosting
        fn take_order() {
            //
        }

        fn serve_order() {
            //
        }

        fn take_payment() {
            //
        }
    }
}

// We can bring a module into scope using the use keyword
// We can also rename it using as
use crate::front_of_the_house::hosting as host;

pub fn eat_at_restaurant() {
    // This is an absolute path to the add_to_wait_list function
    // We can't access it directly due to it being private
    // We needed to make the hosting module and add_to_wait_list function public to be able to access it
    // We can reduce this by starting from the import above
    host::add_to_wait_list();
    // This is a relative path to the add_to_wait_list function
    host::add_to_wait_list();
}
