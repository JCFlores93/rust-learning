fn main() {
    let num: Option<i32> = Some(5);
    let text: Option<&str> = Some("Hello");
    println!("{:?} {:?}", num, text);

    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}", six, none);

    let some_u8 = Some(3);
    if let Some(3) = some_u8 {
        println!("three");
    } else  if let Some(4) = some_u8 {
        println!("four");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

enum UsState{
    Alaska,
    Arizona
} 

fn value_in_cents(c: Coin) -> u32{
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        // Default
        _ => None
    }
}