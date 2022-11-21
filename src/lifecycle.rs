// promise x's lifecycle greater than y's lifecycle
pub fn lifecycle_one() {
    let x = 5;
    let y = &x;
    println!("y value is {}", y)
}

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn cal_longest() {
    let str1 = String::from("hello");
    let str2 = "world";
    let str3 = longest(str1.as_str(), str2);
    println!("str3 value is : {}", str3);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[test]
fn test_one() {
    let novel = String::from("Call me Oliver, some years ago..");
    let first_sentence = novel.split('.').next()
        .expect("could not found '.'");
    let i = ImportantExcerpt{
        part: first_sentence,
    };
    println!("i value is {}", i.part)
}