use std::io;

fn main() {
q3()
}

fn q1() {
    for n in 1..100 {
        if n % 2 == 0 {
            println!("number: {}", n);
        }        
    }
}

fn q3() {
    let mut number = String::new();
    println!("Ingrese un numero");
    io::stdin().read_line(&mut number).expect("Failed");
    number = number.trim().to_string();
    for ch in number.chars(){
        println!("ch: {}", ch);
    }
    let mut n = 432;
    let mut c = 0;

    while n!= 0 {
        c += 1;
        n = n/10;
    }
    println!("{}", c);
}