use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::Add;

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

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self) {
        println!("Button draw");
    }
}

struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox draw");
    }
}

fn draw_test() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 10,
                height: 10,
                options: vec![String::from("Yes"),String::from("Maybe"),String::from("No")],
            }),
            Box::new(Button {
                width: 10,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };
    screen.run();
}

// associated type
pub trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter{

}

// An associated type can only be implemented once
impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        None
    }
}

#[derive(Debug,PartialEq)]
struct Point{
    x:i32,
    y:i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self,other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot{
    fn fly(&self);
}

trait Wizard{
    fn fly(&self);
}

struct Human;

impl Pilot for Human{
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("waving arms furiously");
    }
}

#[test]
fn same_name_test() {
    let person = Human{};
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[test]
fn full_qualified_syntax() {
    println!("a baby dog is called a {}", Dog::baby_name());
    println!("a baby dog is called a {}", <Dog as Animal>::baby_name())
}

//super trait

trait OutlinePrint: fmt::Display{
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}","*".repeat(len + 4))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{} {}",self.x,self.y)
    }
}

impl OutlinePrint for Point{

}

//Use newt ype to implement an external trait on an external type

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"[{}]",self.0.join(", "))
    }
}

#[test]
fn external_trait_test() {
    let w = Wrapper(vec![String::from("hello"),String::from("world")]);
    println!("w = {}", w);
}









