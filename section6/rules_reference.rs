fn main() {
    let mut s = String::from("Hello");
    // You can create any number of inmutable references.
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    // You can create only one mutable reference in a scope. -> False
    let r4 = &mut s;
    let r5 = &mut s;
    let r6 = &mut s;

    // You cannot create mutable refreneces if your program uses more than
    // one inmutable references
     let r7 = &s;


     // Dangling references
     let s = dangle();
}

fn dangle() -> &String {
    let d: String = String::from("Hello");
    &d;
}// d goes out of scope here.