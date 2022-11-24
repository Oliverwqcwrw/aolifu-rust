#[test]
fn if_let_test() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age:Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color {} as the background ", color);
    } else if is_tuesday{
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using the purple color as the background");
        } else {
            println!("Using the orange color as the background");
        }
    } else {
        println!("Using the blue as the background color");
    }
}

#[test]
fn while_let_test() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("top: {}", top)
    }
}

#[test]
fn for_test() {
    let vector = vec![1,2,3];
    for (index,value) in vector.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

#[test]
fn match_test() {
    let x = 5;
    match x {
        1..= 5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='k' => println!("early ASCII letter"),
        'j'..='z' => println!("later ADCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn pattern_test() {
    let p = Point{
        x: 0,
        y: 7,
    };
    let Point{x: a,y:b} = p;
    assert_eq!(a, 0);
    assert_eq!(b, 7);

    let Point{x,y} = p;
    assert_eq!(x, 0);
    assert_eq!(y, 7);

    match p {
        Point{x,y:7} => println!("y is matched"),
        Point{x:0,y} => println!("x is matched"),
        Point{x,y} => println!("on neither exist"),
    }
}

enum Color{
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}

enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
    ChangeColor2(Color),
}

#[test]
fn match_test_two() {
    let msg = Message::ChangeColor(0,255,255);
    match msg {
        Message::Quit => println!("no data to destructure"),
        Message::Move {x,y} => println!("Move in the direction x {} and in the y direction {}", x, y),
        Message::Write(s) => println!("text message is {}", s),
        Message::ChangeColor(r,g,b) => println!("change color to red {}, green {}, blue {}",r,g,b),
        _ => println!("something else")
    }

    let msg = Message::ChangeColor2(Color::Rgb(0,255,255));
    match msg {
        Message::ChangeColor2(Color::Rgb(r,g,b)) => print!("change the color to red {},green {},blue {}",r,g,b),
        Message::ChangeColor2(Color::Hsv(h,s,v)) => println!("change the color to hue {},saturation {},value {}",h,s,v),
        _ => {}
    }
}

struct Point_Two {
    x:i32,
    y:i32,
}

#[test]
fn pattern_test_two() {
    let ((feet,inches), Point_Two{x,y}) = ((3,10),Point_Two{x:3,y:10});
    println!("feet:{},inches:{} x: {}, y:{}",feet,inches,x,y);
}

#[test]
fn match_test_three() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value,new_setting_value) {
        (Some(_),Some(_)) => println!("can't override an existing customized value"),
        _ => {}
    }

    let numbers = (2,4,6,8,10);
    match numbers {
        (a,_,b,_,c) => println!("Some numbers is {} {} {}",a,b,c),
    }

    // avoid unused warning
    let _num1 = 5;
    let _num2 = 10;

    let s = Some(String::from("a"));
    // No transfer of ownership
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}",s)
}

struct Point_Three {
    x:i32,
    y:i32,
    z:i32,
}

#[test]
fn match_test_four() {
    let point = Point_Three{x:3,y:5,z:7};
    match point {
        Point_Three{x,..} => println!("x value is {}",x),
    }
    let numbers = (1,3,5,7,9);
    match numbers {
        (first,..,last) => println!("Some numbers: {} {}",first,last)
    }
}

#[test]
fn match_guard_test() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched! n = {}", n),
        _ => {}
    }

    let a = 4;
    let b = false;
    match a {
        4 | 5 | 6 if b => println!("yes"),
        _ => println!("no"),
    }
}







