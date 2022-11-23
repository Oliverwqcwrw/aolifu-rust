use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::iterator::print_iterator;
use crate::reference_count::List::{Cons, Nil};
use crate::reference_count::List_Two::{Cons_Two, Nil_Two};
use crate::reference_count::List_Three::{Cons_Three, Nil_Three};

enum List{
    Cons(i32, Rc<List>),
    Nil
}

#[test]
fn reference_count_test() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a =  {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b =  {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c =  {}", Rc::strong_count(&a))
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub trait Messenger{
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
    where T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota");
        }
    }

}

struct MockMessenger {
    send_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            send_messages: RefCell::new(vec![])
        }
    }
}

impl Messenger for MockMessenger {
    fn send(& self, msg: &str) {
        self.send_messages.borrow_mut().push(String::from(msg))
    }
}

#[test]
fn send_over_75_percent_warning_messsage_test() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
}

#[derive(Debug)]
enum List_Two{
    Cons_Two(Rc<RefCell<i32>>, Rc<List_Two>),
    Nil_Two
}

#[test]
fn reference_count_test_two() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons_Two(Rc::clone(&value), Rc::new(Nil_Two)));
    let b = Cons_Two(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons_Two(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List_Three {
    Cons_Three(i32, RefCell<Rc<List_Three>>),
    Nil_Three
}

impl List_Three {
    fn tail(&self) -> Option<&RefCell<Rc<List_Three>>> {
        match self {
            Cons_Three(_, item) => Some(item),
            Nil_Three => None,
        }
    }
}

#[test]
fn reference_count_test_three() {
    let a = Rc::new(Cons_Three(5, RefCell::new(Rc::new(Nil_Three))));
    println!("a initial rc count is {} ", Rc::strong_count(&a));
    println!("a next item is {:?}", a.tail());

    let b = Rc::new(Cons_Three(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count is {}", Rc::strong_count(&b));
    println!("b next item is {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after chhanging a = {}", Rc::strong_count(&a));

    // it will overflow the stack
    // println!("a next item = {:?}", a.tail())
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[test]
fn reference_count_test_four() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent= {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}






