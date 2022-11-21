pub fn num<T>(num: T) -> T {
    num
}

struct Color<T> {
    Name: T,
    Type: T,
}

impl <T> Color<T> {
    fn x(&self) -> &T {
        &self.Name
    }
}

impl Color<i32> {
    fn x2(&self) -> &i32 {
        &self.Type
    }
}

struct Color2<T, U> {
    Name: T,
    Type: U,
}

impl <T,U> Color2<T, U> {
    fn mixup<V, W>(self, other: Color2<V,W>) -> Color2<T, W> {
        Color2 {
            Name: self.Name,
            Type: other.Type,
        }
    }
}

pub fn mixup_color() {
    let color1 = Color2 {
        Name: 4,
        Type: 5,
    };
    let color2 = Color2 {
        Name: String::from("hello"),
        Type: 'h',
    };
    let color3 = color1.mixup(color2);
    println!("color3.Name: {} color3.Type: {}", color3.Name, color3.Type);
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
