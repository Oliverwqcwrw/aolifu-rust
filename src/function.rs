use std::arch::x86_64::_bittest;

pub fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for item in list {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}

pub fn cal_largest() {
    let list = [10, 20, 11, 23, 91];
    let largest = largest(&list);
    println!("The largest number is {}", largest);
}


pub fn largest_three<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn largest_four<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

pub fn largest_five<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn cal_largest_two()  {
    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest_five(&str_list);
    println!("largest element is {}", result);

}