use std::ops::Deref;

#[test]
fn dereference_one() {
    let a = 5;
    let b = &a;
    assert_eq!(5, a);
    assert_eq!(5, *b);
}

#[test]
fn dereference_box() {
    let a = 5;
    let b = Box::new(a);
    assert_eq!(5, a);
    assert_eq!(5, *b);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn dereference_mybox() {
    let a = 5;
    let b = MyBox::new(a);
    assert_eq!(5, a);
    assert_eq!(5, *b);
}

fn hello(name: &str) {
    println!("hello : {}", name)
}

#[test]
fn implicit_dereference() {
    let b = MyBox::new(String::from("oliver"));
    hello(&(*b)[..]);
    hello(&b);
}