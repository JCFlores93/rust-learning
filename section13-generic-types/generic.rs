#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

#[derive(Debug)]
struct Points<T> {
    x: T
}

// Generics in Enum Definition
enum Option<T> {
    Some(T),
    None
}

enum Result<T,E> {
    Ok(T),
    Err(E)
}

impl <T> Point <T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Points<f32> {
    fn number(&self) -> f32 {
        self.x
    }
}

impl Points<i32> {
    fn number(&self) -> i32 {
        self.x
    }
}



fn main() {
    let integer = Point{x: 5 ,y: 10};
    let float = Point{x: 8.0 ,y: 11.1};
    println!("{:?} {:?}" ,integer, float);
    println!("{:?}" ,integer.x());
    let n = Points{x: 2.2};
    println!("{:?}" , n.number());
}