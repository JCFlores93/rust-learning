fn main() {
    let a = String::from("Hello World");
    let r1 = &a[0..5];
    let r2 = &a[0..=5];
    let r3 = &a[..5];
    let r4 = &a[0..];
    let r5 = &a[..];
}