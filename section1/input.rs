use std::io;

fn main() {
    let mut a = String::new();
    println!("Enter a string");
    io::stdin().read_line(&mut a).expect("Failed");
    let a : String = a.trim().parse().expect("Failed");
    println!("{}", a);
}