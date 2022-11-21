use std::collections::HashMap;

pub fn hashmap_add() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("red"), 10);
    map.insert(String::from("black"), 20);

    let color1 = String::from("red");
    let num1 = map.get(&color1);

    match num1 {
        Some(num) => println!("num value is {}", num),
        None => println!("there is no key"),
    }

    map.entry(String::from("pink")).or_insert(30);
}

pub fn create_hashmap_collect() {
    let colors = vec!["red", "black"];
    let nums = vec![10,20];
    let colorMap: HashMap<_, _> = colors.iter().zip(nums.iter()).collect();

    for (k,v) in &colorMap {
        println!("k value is {}, v value is {}", k, v);
    }
}

