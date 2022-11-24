extern crate core;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){
            super::hosting::add_to_waitlist();
        }
    }
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}

pub mod collection;

pub mod string;

pub mod panic;

pub mod guess_number;

pub mod function;

pub mod generic;

pub mod trait_mod;

pub mod lifecycle;

#[cfg(test)]
pub mod test;

pub mod command;

pub mod closure;

pub mod iterator;

pub use self::collection::vector::test_vector;

pub mod box_mod;

pub mod dereference;

pub mod reference_count;

pub mod multiple_thread;

pub mod pattern_match;

pub mod unsafe_rust;