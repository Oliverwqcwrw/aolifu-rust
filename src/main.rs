use std::cmp::Ordering;
use std::io;
use rand::Rng;
use aolifu_rust::collection;
use aolifu_rust::collection::vector;
use aolifu_rust::string;
use aolifu_rust::panic;
use aolifu_rust::function;
use aolifu_rust::generic;
use aolifu_rust::trait_mod;

const TEST_CONSTANT: u32 = 100_000;

fn main() {
    function::cal_largest();

    override_variable();

    different_type_variable();

    tuple();

    array();

    let result = plus_num(6);
    println!("result value is {}", result);

    loop_one();

    while_one();

    array_reverse();

    string_append();

    string::string_move();

    string_deep_clone();

    for_one();

    slice_one();

    new_user();

    cal_area();

    rec_is_hold();

    related_func();

    cal_coin_value();

    match_one_test();

    some_one();

    generic::mixup_color();

    trait_mod::send_tweet();

    println!("constant value is {}", TEST_CONSTANT);
}

struct User {
    username:String,
    email:String,
    sing_in_count:u64,
    active:bool,
}

#[derive(Debug)]
struct Rectangle {
    width:u32,
    high:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.high
    }

    fn is_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.high > other.high;
    }

    // related function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            high: size,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn override_variable() {
    let x = 5;
    let x = x + 1;
    let x =  x * 2;
    println!("x value is : {}", x)
}

fn different_type_variable() {
    let s = "abc";
    let s = s.len();
    println!("s type from str to num, s value is {}", s);
}

fn tuple()  {
    let tup: (i32, f64, u8) = (12, 32.4, 2);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
}

fn array() {
    // let arr = [1, 2, 3, 4, 5];
    // let arr = [3; 5];
    let arr = [3, 3, 3, 3, 3];
    for item in arr.iter() {
        println!("the value is {}", item);
    }
}

fn plus_num(x: i32) -> i32 {
    x + 5
}

fn loop_one() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result value is {}", result)
}

fn while_one() {
    let mut number = 3;
    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFT OFF!")
}

fn array_reverse() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIST OFF!");
}

fn string_append() {
    let mut  s = String::from("Hello");
    s.push_str(", World");
    println!("{}", s);
}

fn string_deep_clone() {
    let s1 = String::from("Hello2");
    let s2 =s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}

fn for_one() {
    let s =  String::from("Hello World");
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", i);
            break;
        }
    }
}

fn slice_one() {
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let  world  = &s[6..11];
    println!("{} {}", hello, world);

    let hello2 = &s[..5];
    let world2 = &s[6..];
    println!("{} {}", hello2, world2);

    let whole = &s[..];
    println!("{}", whole);
}

fn new_user() {
    let mut  user = User {
        email: String::from("11@qq.com"),
        username: String::from("oliver"),
        sing_in_count: 123,
        active: true,
    };
    user.active = false;
    println!("{}", user.active);

    let user2 = User {
        email: String::from("22@qq.com"),
        username: String::from("bob"),
        ..user
    };
    println!("{},{},{},{}", user2.email, user2.username, user2.sing_in_count, user2.active)
}

fn cal_area() {
    let rec = Rectangle {
        width: 30,
        high: 50,
    };
    let area = rec.area();
    println!("area: {}", area);
    println!("rec: {:#?}", rec);
}

fn rec_is_hold() {
    let rect = Rectangle {
        width: 50,
        high: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        high: 20,
    };
    let flag = rect.is_hold(&rect2);
    println!("{}", flag)
}

fn related_func() {
    let rec = Rectangle::square(20);
    println!("{:#?}", rec);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn cal_coin_value() {
    let quarter = Coin::Quarter;
    let quarter_value = value_in_cents(quarter);
    println!("quarter value is {}", quarter_value);
}

fn match_one(v: u8) {
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn match_one_test() {
    let v = 1u8;
    match_one(v);
}

fn some_one() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        Some(7) => println!("seven"),
        Some(9) => println!("nine"),
        _ => (),
    }
}

fn if_let_one() {
    let v = Some(0u8);
    if let Some(3) = v {
        println!("{}", v.iter().len());
    }
}

