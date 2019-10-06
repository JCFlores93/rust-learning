fn main() {
    let mut s = String::from("Hello");
    take(&mut s);
    println!("{} ", s);
}

// We never have ownership
// Borrowing
fn take(s1: &mut String) {
    println!("{} ", s1);
    s1.push_str(" world");
}