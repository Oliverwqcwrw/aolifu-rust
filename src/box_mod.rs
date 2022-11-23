use crate::box_mod::List::{Cons, Nil};

#[test]
fn create_box() {
    let b = Box::new(5);
    println!("b = {}", b)
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn create_list() {
    let list = Cons(1,
        Box::new(Cons(2,
        Box::new(Cons(3,
        Box::new(Nil)))))
    );
}