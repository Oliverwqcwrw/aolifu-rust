pub fn print_iterator() {
    let v1 = vec![1,2,3];
    let iterator1 = v1.iter();
    for item in iterator1 {
        println!("{}", item)
    }
}

#[test]
pub fn iterator_demonstration() {
    let v1 = vec![1,2,3];
    let mut  iterator1 = v1.iter();
    assert_eq!(iterator1.next(), Some(&1));
    assert_eq!(iterator1.next(), Some(&2));
    assert_eq!(iterator1.next(), Some(&3));
}

#[test]
pub fn iterator_sum() {
    let v1 = vec![1,2,3];
    let sum: i32 = v1.iter().sum();
    println!("sum: {}", sum)
}

#[test]
pub fn iterator_adapter() {
    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4])
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// take ownership
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        }
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(in_my_size, vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        }

    ])
}

struct Counter{
    count: u32
}

impl Counter {
    fn New() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item  = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut  counter = Counter::New();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::New()
        .zip(Counter::New().skip(1))
        .map(|(a,b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}

enum Message {
    Hello{
        id:i32,
    }
}

fn match_at_test() {
    let msg = Message::Hello {id:5};
    match msg {
        Message::Hello {
            id:id_variable @3..=7,

        } => println!("Found an id in range: {}", id_variable),
        Message::Hello {
            id: 10..=12
        } => println!("Found an id in another range"),
        Message::Hello {
            id
        } => println!("Found some other id: {}", id),

    }
}














