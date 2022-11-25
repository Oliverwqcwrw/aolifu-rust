
pub fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for item in list {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}

pub fn cal_largest() {
    let list = [10, 20, 11, 23, 91];
    let largest = largest(&list);
    println!("The largest number is {}", largest);
}


pub fn largest_three<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn largest_four<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

pub fn largest_five<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn cal_largest_two()  {
    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest_five(&str_list);
    println!("largest element is {}", result);
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}

pub struct Post{
    content: String,
}

pub struct DraftPost{
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct PendingReviewPost{
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[test]
fn post_test() {
    let mut post = Post::new();
    post.add_text("I ate a salad for launch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for launch today", post.content)
}

// type alias

type Kilometers = i32;

#[test]
fn type_alias_test() {
    let x:i32 = 5;
    let y:Kilometers = 10;
    println!("x + y = {}",x + y);
}

// function pointer

fn add_one(x:i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn function_pointer_test() {
    let answer = do_twice(add_one, 5);
    println!("answer is {}", answer);
}

fn function_pointer_test_two() {
    let list_of_numbers = vec![1,2,3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    let list_of_numbers = vec![1,2,3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}











