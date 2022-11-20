use std::cmp::Ordering;
use std::io;
use rand::Rng;

const TEST_CONSTANT: u32 = 100_000;

fn main() {
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

    string_move();

    string_deep_clone();

    for_one();

    slice_one();

    guess_number();

    println!("constant value is {}", TEST_CONSTANT);
}

fn guess_number() {
    println!("guess number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret number is：{}", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("can't read line");
        println!("you guess number is：{}", guess);
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break
            },
        }
    }
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

fn string_move() {
    let s1  = String::from("Hello");
    let s2 = s1;
    println!("s2: {}", s2);
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
