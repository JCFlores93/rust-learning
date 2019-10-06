fn main() {
    let mut s = String::from("Hello");
    take(s);
    println!("{} ", s);
}

// Return ownership back
fn take(s1: String) -> String {
    println!("{} ", s1);
    s1;
}