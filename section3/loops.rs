fn main() {
    let mut n = 0;
    loop {
        if n < 5 {
            println!("Hello loop");
            n += 3;
        }else {
            break;
        }  
    }

    let mut n = 1;
    while n <= 5 {
        println!("Hello while");
        n += 1;
    }

    for n in 1..10 {
         println!("Hello for {}", n);
    }
}