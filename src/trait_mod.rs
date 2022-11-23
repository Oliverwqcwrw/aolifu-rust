use std::arch::x86_64::_bittest;
use std::fmt::{Debug, Display};
use std::iter::Sum;

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        format!("Read more form {}", self.summarize())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn send_tweet() {
    let tweet = Tweet{
        username: String::from("Oliver"),
        content: String::from("this is a good day"),
        reply: true,
        retweet: false,
    };
    let summary = tweet.summarize();
    println!("summary is {}", summary);
}

pub fn notify(item: impl Summary) {
    println!("breaking news ! {}", item.summarize())
}

pub fn notify_two<T: Summary>(item: T) {
    println!("breaking news! {}", item.summarize())
}

pub fn notify_three<T: Summary + Display>(item: T) {
    println!("breaking news! {}", item.summarize())
}

pub fn notify_four(item: impl Summary + Display) {
    println!("breaking news! {}", item.summarize())
}

pub fn notify_five<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("breaking news ! {}", a.summarize())
}

pub fn notify_six<T, U>(a: T, b: U) -> String
    where T: Summary + Display,
        U: Clone + Debug,
{
    format!("breaking news! {}", a.summarize())
}

pub fn trait_return_notify() -> impl Summary {
    NewsArticle{
        headline: String::from("headline_test"),
        location: String::from("location_test"),
        author: String::from("author_test"),
        content: String::from("content_test"),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// trait bound without condition
impl<T> Pair<T>{
    fn New(x: T, y: T) -> Self {
        Pair{
            x, y,

        }
    }
}

// trait bound with condition
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest num is x:{}", self.x);
        } else {
            println!("The largest num is y:{}", self.y);
        }
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data)
    }
}

#[test]
fn drop_trait_test() {
    let c1 = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let c2 = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c1);
    println!("CustomSmartPointer created")
}