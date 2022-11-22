pub fn string_move() {
    let s1  = String::from("Hello");
    let s2 = s1;
    println!("s2: {}", s2);
}

pub fn create_string() {
    let s1 = String::new();
    let s2 = "Hello World";
    let s3 = s2.to_string();
    let s4 = String::from("Hello World");
    println!("{}, {}, {}, {}", s1, s2, s3, s4);
}

pub fn update_string() {
    let mut  s1 = String::from("hello");
    s1.push_str(" World");
    s1.push('a');
    println!("s1 value is {}", s1);

    let s2 = String::from("123");
    let s3 = s1 + &s2 ;
    println!("s3 value is {}", s3);
}

pub fn format_string() {
    let s1 = String::from("tic");
    let s2 = String::from("tib");
    let s3 = String::from("tia");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s4 value is {}", s4);
}

pub fn traverse_string() {
    let s1 = "abc";
    for b in s1.bytes() {
        println!("byte value is {}", b);
    }

    for ch in s1.chars() {
        println!("char value is {}", ch);
    }
}

pub fn split_string() {
    let hello = "hello";
    let s = &hello[0..4];
    println!("s value is {}", s);
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