fn main() {
    let s1 = "Hello";
    let s2 = "Bye";
    let result = longest(s1, s2);
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { 
        x
    } else { 
        y
    }
}