use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn New(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must between 1 and 100, got {}", value);
        }
        Guess{value}
     }

    pub fn value(&self) -> i32 {
        self.value
    }
 }

pub fn guess_number() {
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