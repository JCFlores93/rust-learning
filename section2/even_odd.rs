use std::io;

fn main(){
    let a = 10;
    if a % 2 == 0 {
        println!("Even");
    } else {
         println!("Odd");
    }
    let mut ch = String::new();
    println!("Are your friends comming?");
    io::stdin().read_line(&mut ch).expect("Failed");
    ch = ch.trim().to_string();
    if ch == "y" {
        println!("Yes.They are comming");
    } else { 
        println!("Stay at home");
    }
    let mut name = String::new();
    let mut age = String::new();
    let mut ch = String::new();
    io::stdin().read_line(&mut name).expect("Failed");
    io::stdin().read_line(&mut age).expect("Failed");
    let age: i32 = age.trim().parse().expect("Failed");
    println!("Do you want to create an account");
    io::stdin().read_line(&mut ch).expect("Failed");
    ch = ch.trim().to_string();
    if ch == "y"{
        if age < 10 {
        println!("Your age is less");
    } else { 
        println!("Your account is created");
        println!("Do you want to upload photo");
        ch.clear();
        io::stdin().read_line(&mut ch).expect("Failed");
        ch = ch.trim().to_string();
        if ch == "y"{
            if age < 13 {
                println!("You cannot upload photo");
            }else {
                println!("You can upload photo");
            }
        }else if ch != "y"{
            println!("Thanks for visiting us");
        }
    }
}
}