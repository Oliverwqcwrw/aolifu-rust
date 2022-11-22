use std::thread;
use std::time::Duration;

struct Cacher<T>
 where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn New(calculation: T) -> Cacher<T> {
        Cacher{
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| -> u32 {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else{
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minuted!", expensive_closure(intensity));
        }
    }
}

pub fn generate_workout_two(intensity: u32, random_number: u32) {
    let mut expensive_closure= Cacher::New(|num| -> u32 {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today do {} push ups !", expensive_closure.value(intensity));
        println!("Next do {} sit ups", expensive_closure.value(intensity))
    } else{
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minuted!", expensive_closure.value(intensity));
        }
    }
}

#[test]
fn generate_workout_test() {
    generate_workout_two(2, 3);
}

// Automatically infer parameters and return values
pub fn infer_parameter_return() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
}