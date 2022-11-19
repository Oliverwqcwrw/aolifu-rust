use std::cmp::Ordering;
use std::io;
use rand::Rng;

const TEST_CONSTANT: u32 = 100_000;

fn main() {
    override_variable();
    guess_number();
    println!("constant is {}", TEST_CONSTANT);
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
