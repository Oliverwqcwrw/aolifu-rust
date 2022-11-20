enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn test_vector() {
    println!("Vector")
}

pub fn operate_vector() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
}

pub fn create_vector() {
    let v = vec![1,2,3,4];
    for item in v.iter() {
        println!("value is {}", item);
    }
}

pub fn create_enum_vector() {
    let v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(2.0),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

}

pub fn read_vector() {
    let v = vec![1,2,3,4];
    let third: &i32 = &v[2];
    println!("third element is {}", third);

    match v.get(2) {
        Some(third) => println!("third element valuue is {}", third),
        None => println!("there is no third element"),
    }
}

pub fn traverse_vector() {
    let v = vec![1,2,3,4];
    for i in &v {
        println!("element is {}", i);
    }
}