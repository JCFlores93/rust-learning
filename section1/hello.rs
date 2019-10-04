const MAX: i32 = 10;
fn main() {
    let a: u32 = 1;
    let b: char = 'c';
    println!("Hello, world! {}", b);
    let mut c = 12;
    c = 20;
    println!("{}", c);
    println!("{}", MAX);
    let d = "Hello";
    println!("{}", d);
    // Asignar un nuevo espacio de memoria
    // let e = String::new();
    // e = String::from('Hello');
    let mut f = String::from("Hello");
    f.push_str("Hello");
    // e = String::from('Hello');
    println!("{}", f);

}